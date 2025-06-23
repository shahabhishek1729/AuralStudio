use super::ident::{anglicize_expr, anglicize_piece, eval_expr, IDGraph, Ident};
use crate::{
    addr,
    digraph::{
        address::{Address, Addressable},
        parser::NodeKind,
        state::{Canvas, CursorDir, CursorError},
    },
    static_analysis::ident::format_expr,
};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Debugger {
    pub(crate) state: Canvas,
    call_stack: Vec<Address>,
}

#[derive(Debug, Error)]
pub(crate) enum DebuggerError {
    #[error("Unexpected error")]
    CursorError(#[from] CursorError),
    #[error("Found an unexpected break or continue")]
    UnexpectedCtrl,
}

#[derive(Debug, Clone)]
pub(crate) enum StackAction {
    // Used for OUTPUT and VARDECL, continue on to the next line of code
    NEXT,
    // Used for BREAK/CONTINUE, specifies true if break, false if continue.
    CTRLFLOW(bool),
    // Used for RETURNs, specifies which kind of node you are breaking from.
    RETURN,
    // Moves into a child block, with a usize representing block idx (0/1 for CONDTL, 0 for loops).
    DRILL(usize),
    // Used with FNCALL, jumps into another function.
    JUMP(String),
}

impl StackAction {
    pub(crate) fn execute(&self, debugger: &mut Debugger) -> Result<bool, DebuggerError> {
        match self {
            StackAction::NEXT => match debugger.state.navigate(CursorDir::DOWN) {
                Ok(addr) => {
                    let hash = debugger.state.graph.get_hash();
                    let last_ix = debugger.call_stack.len() - 1;
                    let node_loc = addr.coerce(&hash)?;

                    debugger.call_stack[last_ix] = node_loc.clone();
                    debugger.state.block_loc = node_loc.clone();
                    debugger.state.node_loc = node_loc;
                }
                Err(crate::digraph::state::CursorError::InvalidMotion(_)) => {
                    if debugger.call_stack.len() == 1 {
                        // Program has terminated
                        return Ok(true);
                    }
                    debugger.call_stack.pop();
                    debugger.state.block_loc = debugger.call_stack.last().expect("len > 0").clone();
                    debugger.state.coerce()?;
                    return self.execute(debugger);
                }
                _ => unreachable!(),
            },
            StackAction::CTRLFLOW(is_break) => {
                let hash = debugger.state.graph.get_hash();
                let addr = debugger.call_stack.last().unwrap();
                let mut curr_node = hash.get(addr).unwrap();
                while curr_node.kind != NodeKind::FORLOOP && curr_node.kind != NodeKind::WHLLOOP {
                    debugger.call_stack.pop();
                    let Some(ref last) = debugger.call_stack.last() else {
                        return Err(DebuggerError::UnexpectedCtrl);
                    };
                    curr_node = hash.get(last).unwrap();
                }

                if *is_break {
                    return Self::NEXT.execute(debugger);
                }
            }
            StackAction::RETURN => {
                if debugger.call_stack.len() == 1 {
                    // Program has terminated
                    return Ok(true);
                }
                debugger.call_stack.pop();
                debugger.state.block_loc = debugger.call_stack.last().expect("len > 0").clone();
                debugger.state.coerce()?;
            }
            StackAction::DRILL(ix) => {
                let new_addr = debugger.state.navigate(CursorDir::DOWN)?;
                debugger.state.block_loc = new_addr;
                debugger.state.coerce()?;

                if *ix == 1usize {
                    // We are moving to the "no" branch of a conditional, so D, O, R movement.
                    let new_addr = debugger.state.navigate(CursorDir::OUT).unwrap();
                    debugger.state.block_loc = new_addr;
                    // No need to coerce when moving out, node_loc hasn't changed

                    let new_addr = debugger.state.navigate(CursorDir::RIGHT).unwrap();
                    debugger.state.block_loc = new_addr;
                    debugger.state.coerce()?;
                } else {
                    assert_eq!(*ix, 0usize, "cannot DRILL into an index that is not 0 or 1");
                }
                debugger.call_stack.push(debugger.state.node_loc.clone());
            }
            StackAction::JUMP(_) => todo!(),
        }
        Ok(false)
    }
}

impl Debugger {
    pub(crate) fn new(state: Canvas) -> Debugger {
        return Self {
            state,
            call_stack: vec![addr!(0, 0, 0)],
        };
    }

    pub(crate) fn explain(&self, graph: &mut IDGraph) -> (String, StackAction) {
        let hash = self.state.graph.get_hash();
        let Some(node) = hash.get(&self.state.node_loc) else {
            unreachable!();
        };

        match node.kind {
            NodeKind::FNDEF => todo!(),
            NodeKind::VARDECL => {
                let result = format_expr(&self.state, &node.pieces[2..], graph);
                let evaled = format_strings(eval_expr(&self.state, &node.pieces[2..], graph));

                let curr_ident = graph.get_hash_mut();
                let Some(curr_ident) = curr_ident.get(&self.state.node_loc) else {
                    panic!();
                };
                if let Ident::Var { ref mut val, .. } = *curr_ident.borrow_mut() {
                    *val = Some(evaled.clone());
                }
                return (
                    format!(
                        "Set variable {} equal to {}",
                        anglicize_piece(&node.pieces[0]),
                        result
                    ),
                    StackAction::NEXT,
                );
            }
            NodeKind::OUTPUT => (
                format!(
                    "Wrote on the screen {}",
                    format_expr(&self.state, &node.pieces[..], graph)
                ),
                StackAction::NEXT,
            ),
            NodeKind::CONDTL => {
                if eval_expr(&self.state, &node.pieces[..], graph) == "True" {
                    (
                        format!("Is {}? {}", anglicize_expr(&node.pieces[..]), "Yes"),
                        StackAction::DRILL(0),
                    )
                } else {
                    (
                        format!("Is {}? {}", anglicize_expr(&node.pieces[..]), "No"),
                        StackAction::DRILL(1),
                    )
                }
            }
            NodeKind::CONDTLY => ("Entered the left branch".into(), StackAction::NEXT),
            NodeKind::CONDTLN => ("Entered the right branch".into(), StackAction::NEXT),
            NodeKind::FORLOOP => todo!(),
            NodeKind::WHLLOOP => (
                format!(
                    "While {}, which it {} is.",
                    format_expr(&self.state, &node.pieces[..], graph),
                    if eval_expr(&self.state, &node.pieces[..], graph) == "true" {
                        "still"
                    } else {
                        "no longer"
                    }
                ),
                StackAction::DRILL(0),
            ),
            NodeKind::BREAK => (
                "Hit a break, exiting this loop".into(),
                StackAction::CTRLFLOW(true),
            ),
            NodeKind::CONTINUE => (
                "Hit a continue, starting another loop iteration".into(),
                StackAction::CTRLFLOW(false),
            ),
            NodeKind::RETURN => (
                format!(
                    "Returned {}",
                    format_expr(&self.state, &node.pieces[..], graph)
                ),
                StackAction::RETURN,
            ),
            NodeKind::FNCALL => todo!(),
            NodeKind::GRABPKG => (
                format!("Grabbed the package {}", anglicize_piece(&node.pieces[0])),
                StackAction::NEXT,
            ),
            NodeKind::PENDING => (
                "Oops! I hit placeholder code you haven't started".into(),
                StackAction::NEXT,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        addr,
        digraph::{
            address::Addressable,
            parser::Parser,
            state::{ADMode, Canvas},
        },
        static_analysis::{debugger::Debugger, ident::IDGraph},
    };

    #[test]
    fn explain_var() {
        let SOURCE = "define start of args\nlet a be 1\nlet b be 2\nlet c be 3\nlet d be 4\nlet result be a plus b plus c plus d\nif result equals 10\ndone if\notherwise\ndone otherwise\ndone define";
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
            err: None,
        };

        let mut idg = IDGraph::from_state(&state);
        idg.populate_valid_idents();

        let mut debugger = Debugger::new(state);

        let (expl, act) = debugger.explain(&mut idg);
        assert_eq!(expl, "Set variable a equal to 1");
        act.execute(&mut debugger).unwrap();
        idg.populate_valid_idents();

        let (expl, act) = debugger.explain(&mut idg);
        assert_eq!(expl, "Set variable b equal to 2");
        act.execute(&mut debugger).unwrap();
        idg.populate_valid_idents();

        let (expl, act) = debugger.explain(&mut idg);
        assert_eq!(expl, "Set variable c equal to 3");
        act.execute(&mut debugger).unwrap();
        idg.populate_valid_idents();

        let (expl, act) = debugger.explain(&mut idg);
        assert_eq!(expl, "Set variable d equal to 4");
        act.execute(&mut debugger).unwrap();
        idg.populate_valid_idents();

        let json = serde_json::to_string_pretty(&idg).unwrap();
        dbg!(json);

        let (expl, act) = debugger.explain(&mut idg);
        assert_eq!(
            expl,
            "Set variable result equal to a plus b plus c plus d, which is 10"
        );
        act.execute(&mut debugger).unwrap();
        idg.populate_valid_idents();

        let (expl, act) = debugger.explain(&mut idg);
        assert_eq!(expl, "Is result equal to 10? Yes");
        act.execute(&mut debugger).unwrap();
        idg.populate_valid_idents();
        assert!(false);
    }

    // #[test]
    // fn explain_condtl() {
    //     let SOURCE = "define start of args\nlet a be 1\nlet b be 2\nlet c be a plus b\nif c equals 3\ndone if\notherwise\ndone otherwise\ndone define";
    //     let mut parser = Parser::new(String::from(SOURCE)).unwrap();
    //     let mut nodes = parser.parse().unwrap();
    //     (&mut nodes[..]).fill_addr();

    //     let state = Canvas {
    //         filename: "".into(),
    //         block_loc: addr!(0, 0, 1),
    //         node_loc: addr!(0, 0, 1)
    //             .coerce(&nodes.get_hash())
    //             .expect("Coercion should work"),
    //         mode: ADMode::VIEW,
    //         graph: nodes.to_vec(),
    //         piece_ix: None,
    //         output: None,
    //         err: None,
    //     };

    //     let mut debugger = Debugger::new(state);

    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Set variable a equal to 1");
    //     act.execute(&mut debugger).unwrap();

    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Set variable b equal to 2");
    //     act.execute(&mut debugger).unwrap();

    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Set variable c equal to a plus b, which is 3");
    //     act.execute(&mut debugger).unwrap();

    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Is c equal to 3? Yes");
    //     act.execute(&mut debugger).unwrap();
    // }

    // #[test]
    // fn explain_full() {
    //     let SOURCE = "define start of args\nlet a be 1\nlet b be 2\nif a equals b\ndone if\notherwise\ndone otherwise\ndone define";
    //     let mut parser = Parser::new(String::from(SOURCE)).unwrap();
    //     let mut nodes = parser.parse().unwrap();
    //     (&mut nodes[..]).fill_addr();

    //     let mut state = Canvas {
    //         filename: "".into(),
    //         block_loc: addr!(0, 0, 1),
    //         node_loc: addr!(0, 0, 1)
    //             .coerce(&nodes.get_hash())
    //             .expect("Coercion should work"),
    //         mode: ADMode::VIEW,
    //         graph: nodes.to_vec(),
    //         piece_ix: None,
    //         output: None,
    //         err: None,
    //     };

    //     let mut debugger = Debugger::new(state);
    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Set variable a equal to 1");
    //     act.execute(&mut debugger).unwrap();

    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Set variable b equal to 2");
    //     act.execute(&mut debugger).unwrap();

    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Is a equal to b? No");
    //     act.execute(&mut debugger).unwrap();

    //     let (expl, act) = debugger.explain();
    //     assert_eq!(expl, "Entered the right branch");
    //     act.execute(&mut debugger).unwrap();
    // }
}

// Takes in a string of the form "'...'", and outputs a string of the form "\"...\""
fn format_strings(input: String) -> String {
    if input.len() >= 2 && input.starts_with('\'') && input.ends_with('\'') {
        let result = &input[1..input.len() - 1];
        format!("\"{result}\"")
    } else {
        input
    }
}
