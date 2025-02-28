use crate::digraph::parser::{Node, Piece};
use crate::digraph::{address::Addressable, parser::NodeKind, state::CursorState};
use crate::static_analysis::ident::IDGraph;

struct Analyzer;
impl Analyzer {
    pub fn analyze(state: CursorState, id_graph: IDGraph) -> Result<(), SemanticError> {
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
                    NodeKind::RETURN | NodeKind::BREAK | NodeKind::CONTINUE => {
                        return Err(SemanticError::UnreachableCode)
                    }
                    _ => continue,
                }
            }
        }

        fn _analyze_piecewise(
            pieces: &Vec<Piece>,
            curr_node: &Node,
            id_graph: &IDGraph,
            state: &CursorState,
        ) -> Result<(), SemanticError> {
            let (hash, args_hash) = id_graph.get_hash();
            for (i, piece) in pieces.iter().enumerate() {
                match piece {
                    crate::digraph::parser::Piece::IDENT(name)
                        if curr_node.kind != NodeKind::FNDEF =>
                    'inner: {
                        if curr_node.kind == NodeKind::VARDECL && i == 0 {
                            // This is fine
                            break 'inner;
                        }
                        let Some(ident) = hash.get(&state.block_loc) else {
                            // TODO: Handle this better
                            panic!()
                        };
                        if !ident.is_valid() {
                            return Err(SemanticError::UseBeforeDef(name.into()));
                        }
                    }
                    crate::digraph::parser::Piece::FNCALL(pieces) => {
                        _analyze_piecewise(pieces, curr_node, id_graph, state)?;
                        let n_args_supp = pieces.len() - 1;
                        let Piece::IDENT(ref fn_name) = pieces[0] else {
                            // TODO: Handle this better
                            panic!()
                        };
                        let Some(&n_args_req) = args_hash.get(fn_name) else {
                            panic!()
                        };
                        if n_args_supp != n_args_req {
                            return Err(SemanticError::UnmatchedSignature(n_args_supp, n_args_req));
                        }
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
#[derive(Debug, PartialEq)]
pub(crate) enum SemanticError {
    /// Code found directly below a break, a continue or a return that can never be executed.
    /// XXX: Doesn't strictly need to be an error, yet likely accidental on the programmer's part.
    UnreachableCode,
    /// When a variable is used in an expression or a function is called where the identifier does
    /// not match a valid entry in the identifier digraph.
    UseBeforeDef(String),
    /// When a function is called with the incorrect number of arguments (NOTE: not type checked)
    UnmatchedSignature(usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        addr,
        digraph::{
            address::Addressable,
            parser::Parser,
            state::{ADMode, CursorState},
        },
        static_analysis::ident::IDGraph,
    };

    #[test]
    fn use_before_def() {
        let SOURCE = "define f of x\nlet x be y\ndone define";
        let mut parser = Parser::new(String::from(SOURCE)).unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = CursorState {
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
        let result = Analyzer::analyze(state, dag);
        assert_eq!(result, Err(SemanticError::UseBeforeDef("y".into())));
    }

    #[test]
    fn arg_mismatch() {
        let SOURCE =
            "define f of x\nlet x be 2\ndone define\ndefine g of x\nlet x be f of 2 and 3 done";
        let mut parser = Parser::new(String::from(SOURCE)).unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = CursorState {
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
        let result = Analyzer::analyze(state, dag);
        assert_eq!(result, Err(SemanticError::UnmatchedSignature(2, 1)));
    }
}
