use crate::digraph::parser::{Node, Piece};
use crate::digraph::{address::Addressable, parser::NodeKind, state::Canvas};
use crate::static_analysis::ident::IDGraph;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) struct Analyzer;
impl Analyzer {
    pub(crate) fn analyze(state: &Canvas, id_graph: &IDGraph) -> Result<(), SemanticError> {
        let hash = state.graph.get_hash();
        let curr_node = hash
            .get(&state.block_loc)
            .expect("Current node can never be missing");

        if let Some(parent_node) = hash.get(&curr_node.parent_addr) {
            for child in &parent_node.children {
                if child.addr == state.block_loc {
                    break;
                }
                match child.kind {
                    NodeKind::BREAK => return Err(SemanticError::UnreachableCode("break".into())),
                    NodeKind::CONTINUE => {
                        return Err(SemanticError::UnreachableCode("continue".into()))
                    }
                    NodeKind::RETURN => {
                        return Err(SemanticError::UnreachableCode("return".into()))
                    }
                    _ => continue,
                }
            }
        }

        fn _analyze_piecewise(
            pieces: &Vec<Piece>,
            curr_node: &Node,
            id_graph: &IDGraph,
            state: &Canvas,
        ) -> Result<(), SemanticError> {
            let (hash, args_hash) = id_graph.get_hash();
            for (i, piece) in pieces.iter().enumerate() {
                match piece {
                    crate::digraph::parser::Piece::IDENT(name)
                        if curr_node.kind != NodeKind::FNDEF =>
                    'inner: {
                        if curr_node.kind == NodeKind::VARDECL && i == 0 {
                            // When declaring variable names, no need to check validity. Naming
                            // conventions are checked when the value is updated because otherwise
                            // the scanner will crash.
                            break 'inner;
                        }
                        let Some(ident) = hash.get(&state.block_loc) else {
                            return Err(SemanticError::UseBeforeDef(name.into()));
                        };

                        if !ident.is_valid(name) {
                            return Err(SemanticError::UseBeforeDef(name.into()));
                        }
                    }
                    crate::digraph::parser::Piece::FNCALL(pieces) => {
                        _analyze_piecewise(pieces, curr_node, id_graph, state)?;
                        let n_args_supp = pieces.len() - 1;
                        let Piece::IDENT(ref fn_name) = pieces[0] else {
                            unreachable!("function calls must start with identifiers");
                        };
                        let Some(&n_args_req) = args_hash.get(fn_name) else {
                            return Err(SemanticError::UseBeforeDef(fn_name.into()));
                        };
                        if n_args_supp != n_args_req {
                            return Err(SemanticError::UnmatchedSignature(
                                fn_name.into(),
                                n_args_supp,
                                n_args_req,
                            ));
                        }
                    }
                    crate::digraph::parser::Piece::LIST(pieces) => {
                        _analyze_piecewise(pieces, curr_node, id_graph, state)?
                    }
                    _ => {}
                }
            }

            Ok(())
        }
        _analyze_piecewise(&curr_node.pieces, curr_node, &id_graph, &state)
    }
}

/// The kinds of semantic errors detectable by AuralStudio during interactive development.
/// NOTE: Doesn't include syntax errors, as these are rendered impossible by digraphs.
#[derive(Debug, Clone, PartialEq, Error, Serialize, Deserialize)]
pub(crate) enum SemanticError {
    /// Code found directly below a break, a continue or a return that can never be executed.
    /// XXX: Doesn't strictly need to be an error, yet likely accidental on the programmer's part.
    #[error("code right below a {0} will never be run.")]
    UnreachableCode(String),
    /// When a variable doesn't have a valid name (e.g., uses a keyword, starts with #, etc.)
    #[error("your variable {0} has an invalid name. Please rename it.")]
    InvalidVarName(String),
    /// When a variable is used in an expression or a function is called where the identifier does
    /// not match a valid entry in the identifier digraph.
    #[error("can't find the variable or function named {0}")]
    UseBeforeDef(String),
    /// When a function is called with the incorrect number of arguments (NOTE: not type checked)
    #[error("the function {0} expected {2} arguments, but you provided {1}.")]
    UnmatchedSignature(String, usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        addr,
        digraph::{
            address::Addressable,
            parser::Parser,
            state::{ADMode, Canvas},
        },
        static_analysis::ident::IDGraph,
    };

    #[test]
    fn use_before_def() {
        let SOURCE = "define f of x\nlet x be y\ndone define";
        let mut parser = Parser::new(String::from(SOURCE)).unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = Canvas {
            filename: "".into(),
            block_loc: addr!(0, 0, 1),
            node_loc: addr!(0, 0, 1)
                .coerce(&nodes.get_hash())
                .expect("Coercion should work"),
            mode: ADMode::VIEW,
            graph: nodes.to_vec(),
            piece_ix: None,
            output: None,
        };

        let dag = IDGraph::from_state(&state);
        let result = Analyzer::analyze(&state, &dag);
        assert_eq!(result, Err(SemanticError::UseBeforeDef("y".into())));
    }

    #[test]
    fn arg_mismatch() {
        let SOURCE =
            "define f of x\nlet x be 2\ndone define\ndefine g of x\nlet x be f of 2 and 3 done";
        let mut parser = Parser::new(String::from(SOURCE)).unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = Canvas {
            filename: "".into(),
            block_loc: addr!(1, 0, 1),
            node_loc: addr!(1, 0, 1)
                .coerce(&nodes.get_hash())
                .expect("Coercion should work"),
            mode: ADMode::VIEW,
            graph: nodes.to_vec(),
            piece_ix: None,
            output: None,
        };

        let dag = IDGraph::from_state(&state);
        dag.populate_valid_idents();
        let result = Analyzer::analyze(&state, &dag);
        assert_eq!(
            result,
            Err(SemanticError::UnmatchedSignature("f".into(), 2, 1))
        );
    }

    #[test]
    fn valid_code() {
        let SOURCE = "define f of x\nlet x be 2\nlet y be x\ndone define";
        let mut parser = Parser::new(String::from(SOURCE)).unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = Canvas {
            filename: "".into(),
            block_loc: addr!(0, 0, 2),
            node_loc: addr!(0, 0, 2)
                .coerce(&nodes.get_hash())
                .expect("Coercion should work"),
            mode: ADMode::VIEW,
            graph: nodes.to_vec(),
            piece_ix: None,
            output: None,
        };

        let dag = IDGraph::from_state(&state);
        dag.populate_valid_idents();
        let result = Analyzer::analyze(&state, &dag);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn valid_code_args() {
        let SOURCE = "define f of x\nlet y be x\ndone define";
        let mut parser = Parser::new(String::from(SOURCE)).unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = Canvas {
            filename: "".into(),
            block_loc: addr!(0, 0, 1),
            node_loc: addr!(0, 0, 1)
                .coerce(&nodes.get_hash())
                .expect("Coercion should work"),
            mode: ADMode::VIEW,
            graph: nodes.to_vec(),
            piece_ix: None,
            output: None,
        };

        let dag = IDGraph::from_state(&state);
        dag.populate_valid_idents();
        let result = Analyzer::analyze(&state, &dag);
        assert_eq!(result, Ok(()));
    }
}
