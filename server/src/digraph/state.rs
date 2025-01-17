use crate::addr;
use crate::check;
use crate::digraph::address::{Address, Addressable};
use crate::digraph::parser::NodeKind;
use crate::digraph::parser::Piece;
use crate::digraph::util::*;
pub(crate) use crate::prelude::CursorError;
use crate::Node;
use anyhow;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

/// All possible directions of motion within a digraph
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum CursorDir {
    /// Globally, moving to the parent block.
    /// Locally, moving to the group directly above.
    UP,
    /// Globally, moving to the leftmost child block.
    /// Locally, moving to the group directly below.
    DOWN,
    /// Globally, moving left to a sibling block.
    /// Locally, moving from the "no" to the "yes" branch.
    LEFT,
    /// Globally, moving right to a sibling block.
    /// Locally, moving from the "yes" to the "no" branch.
    RIGHT,
    /// Globally, moving in from block to root node (i.e., to local scope).
    /// Locally, moving in from a loop/conditional to its body.
    IN,
    /// Globally, this motion is invalid.
    /// Locally, moving out to parent groups.
    OUT,
}

impl std::fmt::Display for CursorDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl CursorDir {
    fn _modifying_val(&self) -> bool {
        match self {
            CursorDir::UP | CursorDir::LEFT => false,
            CursorDir::DOWN | CursorDir::RIGHT => true,
            _ => unreachable!("Cannot retrieve modifying value for a IN, OUT operation"),
        }
    }

    fn _parity_val(&self) -> usize {
        match self {
            CursorDir::UP | CursorDir::DOWN => 1,
            CursorDir::LEFT | CursorDir::RIGHT => 0,
            _ => unreachable!("Cannot retrieve modifying value for a IN, OUT operation"),
        }
    }

    fn _ensure_global_validity(&self, src: &Address) -> anyhow::Result<(), CursorError> {
        // 0.0.0, 1.0.0, 2.0.0.0, etc. -> can't go up anymore without hitting the root
        if *self == CursorDir::UP && src[1..].iter().sum::<usize>() == 0 {
            return Err(CursorError::InvalidMotion(*self));
        }

        // If the length is even, the last index is odd (i.e., represents a level). If this is not
        // true, this must mean there are no levels below the current block in the current subtree.
        if *self == CursorDir::DOWN && src.len() % 2 == 1 {
            return Err(CursorError::InvalidMotion(*self));
        }

        Ok(())
    }

    // Retrieves the current node and its index within its parent
    fn _find_parent_child<'a>(
        &self,
        state: &'a CursorState,
        src: &'a Address,
    ) -> Result<(&'a Node, usize), CursorError> {
        let graph_hash: HashMap<Address, &'a Node> = state.graph.get_hash();
        let src = src.coerce(&graph_hash)?;
        let Some(&node) = graph_hash.get(&src) else {
            return Err(CursorError::InvalidAddress(src.clone()));
        };

        let Some(_) = graph_hash.get(&node.parent_addr) else {
            // This node is a FNDEF/CLSDECL; cannot go up without going out first.
            return Err(CursorError::InvalidMotion(*self));
        };

        let coerced_parent = node.parent_addr.coerce(&graph_hash)?;
        if coerced_parent == node.addr {
            // This only allows for DOWN, and is handled separately in that branch.
            return Err(CursorError::InvalidMotion(*self));
        }

        let Some(parent) = graph_hash.get(&coerced_parent) else {
            return Err(CursorError::InvalidAddress(node.parent_addr.clone()));
        };

        let Some((i, curr_node)): Option<(usize, &Node)> = _filter_children(*parent)
            .enumerate()
            .find(|(_, n)| n.addr == src)
        else {
            // This is an unacceptable local motion
            return Err(CursorError::InvalidMotion(*self));
        };
        assert_eq!(curr_node, node);

        Ok((parent, i))
    }

    fn move_global<'a>(&'a self, state: &CursorState) -> anyhow::Result<Address, CursorError> {
        let src = &state.block_loc;

        self._ensure_global_validity(src)?;

        match self {
            CursorDir::UP | CursorDir::DOWN | CursorDir::LEFT | CursorDir::RIGHT => {
                let mut dst = src.clone().addr;
                let l = dst.len();
                for i in (0..l).filter(|x| x % 2 == self._parity_val()).rev() {
                    // If we're going up, make sure we can and move. Otherwise, go down.
                    check!(self._modifying_val() => dst[i] += 1 ; check!(dst[i] > 0 => dst[i] -= 1 ; continue));
                    let dst = Address::new(dst[0..(i + 1)].to_vec());

                    // Ensure the motion worked (i.e., not going right from the rightmost block).
                    let Ok(mut dst) = dst.coerce(&state.graph.get_hash()) else {
                        return Err(CursorError::InvalidMotion(*self));
                    };
                    let _ = dst.addr.pop();

                    return Ok(dst);
                }
                return Err(CursorError::InvalidMotion(*self));
            }
            CursorDir::IN => {
                let dst = src.coerce(&(state.graph).get_hash())?;
                Ok(dst)
            }
            _ => return Err(CursorError::InvalidMotion(*self)),
        }
    }

    fn move_local<'cur>(&'cur self, state: &CursorState) -> anyhow::Result<Address, CursorError> {
        let src: &Address = &state.block_loc;

        // When going up and down, no coercion - check children for local blocks
        // Left and right involves finding the parent's next child
        // In and out involves directly finding parent and child and moving (up until global block)
        match self {
            CursorDir::UP => {
                let (parent, i) = self._find_parent_child(state, src)?;
                let children: Vec<&Node> = _filter_children(parent).collect();

                // If this is the parent's root child, go out and coerce to the nearest node
                if i < N_ROOT_CHILDREN[&format!("{:?}", parent.kind)] as usize {
                    let dst = CursorDir::OUT.move_local(state)?;
                    let dst = dst.coerce(&(state.graph).get_hash())?;
                    return Ok(dst);
                }
                // Return the parent's previous child -> this should be the one right above
                Ok(children[i - 1].addr.clone())
            }
            CursorDir::DOWN => {
                let graph = &state.graph;
                let graph_hash = graph.get_hash();
                let Some(&node) = graph_hash.get(&src.coerce(&graph_hash)?) else {
                    return Err(CursorError::InvalidAddress(src.clone()));
                };

                // We either need to go to a child node or to the following sibling.
                if N_ROOT_CHILDREN.contains_key(&format!("{:?}", node.kind)[..]) && state._at_node()
                {
                    let Some(n_) = node
                        .children
                        .iter()
                        .filter(|c| !GLOBAL_BLOCKS.contains(&c.kind))
                        .next()
                    else {
                        return Err(CursorError::InvalidMotion(CursorDir::DOWN));
                    };

                    return Ok(n_.addr.clone());
                }

                let (parent, i) = self._find_parent_child(state, src)?;
                let children: Vec<&Node> = _filter_children(parent).collect();
                if i == children.len() - 1 {
                    return Err(CursorError::InvalidMotion(*self));
                }

                // Return the parent's next child -> this should be the one right below
                // NOTE: XXX: Need the returned address always have the same length as `src`?
                Ok(Address::new(
                    children[i + 1].addr.addr[..src.len()].to_vec(),
                ))
            }
            CursorDir::LEFT | CursorDir::RIGHT => {
                let (parent, i) = self._find_parent_child(state, src)?;
                match parent.children[i].kind {
                    NodeKind::CONDTLN if *self == CursorDir::LEFT => Ok(_filter_children(parent)
                        .collect::<Vec<_>>()[i - 1]
                        .addr
                        .clone()),
                    NodeKind::CONDTLY if *self == CursorDir::RIGHT => Ok(_filter_children(parent)
                        .collect::<Vec<_>>()[i + 1]
                        .addr
                        .clone()),
                    _ => Err(CursorError::InvalidMotion(*self)),
                }
            }
            CursorDir::IN => self.move_global(state),
            CursorDir::OUT => {
                let mut src = src.clone();
                let _ = src.addr.pop();
                Ok(src)
            }
        }
    }
}

/// When pieces are pending, there are three possibilities of the next piece we're expecting
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub(super) enum Expecting {
    /// Anything that is or simplifies to a boolean, number, string, list, etc.
    Value,
    /// An operator (takes in 1-2 values and produces a third value)
    Op,
    /// A piece that would begin a new line (typically a keyword)
    Token,
}

impl std::ops::Not for Expecting {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Value => Self::Op,
            _ => Self::Value,
        }
    }
}

/// The modes a user can be in when navigating through the digraph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(super) enum ADMode {
    /// Used to insert code or replace existing code (i.e., modifying the digraph)
    EDIT(Expecting),
    /// Used for moving around in the digraph, running code or any other non-modifying actions.
    VIEW,
    /// Typing with keyboard (no special commands to be executed)
    TYPE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CursorState {
    pub(crate) filename: String,
    pub(crate) block_loc: Address,
    pub(crate) node_loc: Address,
    pub(super) mode: ADMode,
    pub(super) graph: Vec<Node>,
    pub(super) piece_ix: Option<Vec<usize>>,
    pub(super) output: Option<String>,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            filename: "blank.rattle".into(),
            block_loc: addr!(0, 0),
            node_loc: addr!(0, 0, 0),
            graph: vec![],
            mode: ADMode::VIEW,
            piece_ix: None,
            output: None,
        }
    }
}

impl CursorState {
    pub fn navigate(&self, dir: CursorDir) -> Result<Address, CursorError> {
        let graph_hash = self.graph.get_hash();

        if self.block_loc == addr!() {
            match dir {
                CursorDir::DOWN => return Ok(addr!(0, 0)),
                _ => return Err(CursorError::InvalidMotion(dir)),
            }
        }

        let Some(coerced_node) = graph_hash.get(&self.node_loc) else {
            return Err(CursorError::AddrNotFound(self.node_loc.clone()));
        };

        let dst = if !self._at_node() && GLOBAL_BLOCKS.contains(&coerced_node.kind) {
            dir.move_global(&self)?
        } else {
            dir.move_local(&self)?
        };

        Ok(dst.clone())
    }

    pub fn coerce(&mut self) -> Result<(), CursorError> {
        self.node_loc = self.block_loc.coerce(&self.graph.get_hash())?;
        Ok(())
    }

    pub fn _at_node(&self) -> bool {
        self.block_loc == self.node_loc
    }

    pub fn to_rtl(&self) -> String {
        let mut rtl = String::new();
        fn _inner(_node: &Node, _rtl: &mut String) {
            let starter = match _node.kind {
                NodeKind::FNDEF => "define",
                NodeKind::VARDECL => "let",
                NodeKind::OUTPUT => "output",
                NodeKind::CONDTL => "if",
                NodeKind::CONDTLY => "if",
                NodeKind::CONDTLN => "otherwise",
                NodeKind::FORLOOP => "for",
                NodeKind::WHLLOOP => "while",
                NodeKind::BREAK => "break",
                NodeKind::CONTINUE => "continue",
                NodeKind::RETURN => "return",
                NodeKind::FNCALL => "call",
                NodeKind::GRABPKG => "grab",
                NodeKind::PENDING => "pretend",
            };

            if _node.kind != NodeKind::CONDTLY {
                _rtl.push_str(&format!(
                    "{starter}{}\n",
                    _node
                        .pieces
                        .iter()
                        .map(|p| piece_to_str(p))
                        .fold(String::new(), |curr, next| format!("{} {}", curr, next)),
                ));
            }

            for _child in &_node.children {
                _inner(_child, _rtl);
            }

            if _node.children.len() > 0 && _node.kind != NodeKind::CONDTL {
                _rtl.push_str(&format!("done {starter}\n"));
            }

            fn piece_to_str(piece: &Piece) -> String {
                match piece {
                    crate::digraph::parser::Piece::IDENT(s) => s.into(),
                    crate::digraph::parser::Piece::NUMBER(n) => n.to_string(),
                    crate::digraph::parser::Piece::TEXT(s) => format!("text {} done", s),
                    crate::digraph::parser::Piece::BOOL(b) => b.to_string(),
                    crate::digraph::parser::Piece::NOTHING => "nothing".into(),
                    crate::digraph::parser::Piece::OP(op) => match op {
                        crate::digraph::parser::OpKind::ADD => "plus".into(),
                        crate::digraph::parser::OpKind::SUB => "minus".into(),
                        crate::digraph::parser::OpKind::MUL => "times".into(),
                        crate::digraph::parser::OpKind::DIV => "over".into(),
                        crate::digraph::parser::OpKind::MOD => "modulo".into(),
                        crate::digraph::parser::OpKind::EQ => "equals".into(),
                        crate::digraph::parser::OpKind::NE => "not equals".into(),
                        crate::digraph::parser::OpKind::GT => "greater than".into(),
                        crate::digraph::parser::OpKind::LT => "less than".into(),
                        crate::digraph::parser::OpKind::GE => "greater than equals".into(),
                        crate::digraph::parser::OpKind::LE => "less than equals".into(),
                        crate::digraph::parser::OpKind::ASSN => "be".into(),
                        crate::digraph::parser::OpKind::AND => "and".into(),
                        crate::digraph::parser::OpKind::OR => "or".into(),
                        crate::digraph::parser::OpKind::NOT => "not".into(),
                        crate::digraph::parser::OpKind::IN => "in".into(),
                        crate::digraph::parser::OpKind::DOT => "dot".into(),
                        crate::digraph::parser::OpKind::AT => "at".into(),
                    },
                    crate::digraph::parser::Piece::FNCALL(internals) => {
                        let fn_name = piece_to_str(&internals[0]);
                        let mut args = String::new();
                        for (i, arg) in internals.iter().skip(1).enumerate() {
                            if i > 0 && internals[i].resolves_to_val() && arg.resolves_to_val() {
                                args.push_str("and ");
                            }
                            args.push_str(&piece_to_str(&arg));
                            args.push(' ')
                        }
                        format!("{} of {} done", fn_name, args)
                    }
                    crate::digraph::parser::Piece::LIST(internals) => {
                        let mut args = String::new();
                        for arg in internals.iter().skip(1) {
                            args.push_str(&piece_to_str(&arg));
                            args.push(' ');
                        }
                        format!("list {} done", args)
                    }
                    crate::digraph::parser::Piece::PendingVal => "pretend".into(),
                    crate::digraph::parser::Piece::PendingOp => "pretend".into(),
                }
            }
        }

        for node in &self.graph {
            _inner(node, &mut rtl);
        }
        rtl
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addr;
    use crate::digraph::address::Addressable;
    use crate::digraph::parser::Parser;
    use crate::digraph::state::{CursorError::InvalidMotion, CursorState};

    macro_rules! move_cursor {
        // Move sequences that result in an attempted move "off the graph" should return errors
        ($($dir:tt),+ _in_ $src:ident -> $err:tt) => {{
            let mut parser = Parser::new(String::from($src)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();
            let mut state = CursorState {
                filename: "".into(),
                block_loc: addr!(0, 0),
                node_loc: addr!(0, 0).coerce(&nodes.get_hash()).expect("Coercion should work"),
                mode: ADMode::VIEW,
                graph: nodes.to_vec(),
                piece_ix: None,
                output: None,
            };
            let mut failed = false;
            $(
                let coerced = state.block_loc.coerce(&(state.graph).get_hash()).expect("Coercion should work");
                let dst = if coerced != state.block_loc && GLOBAL_BLOCKS.contains(&(state.graph).get_hash().get(&coerced).expect("Retrieval should work").kind) {
                    move_cursor!($dir).move_global(&state)
                } else {
                    move_cursor!($dir).move_local(&state)
                };
                failed = failed || matches!(dst, Err($err(_)));
                if let Ok(dst) = dst {
                    state.block_loc = dst;
                    let _ = state.coerce().expect("Post-motion coercion should succeed");
                }
            )+
            assert!(failed);
        }};
        ($($dir:tt),+ _in_ $src:ident -> <$($id:literal),+>) => {{
            let mut parser = Parser::new(String::from($src)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();
            let mut state = CursorState {
                filename: "".into(),
                block_loc: addr!(0, 0),
                node_loc: addr!(0, 0).coerce(&nodes.get_hash()).expect("Coercion should work"),
                mode: ADMode::VIEW,
                graph: nodes.to_vec(),
                piece_ix: None,
                output: None,
            };
            $(
                let coerced = state.block_loc.coerce(&(state.graph).get_hash()).expect("Coercion should work");
                let dst = if coerced != state.block_loc && GLOBAL_BLOCKS.contains(&(state.graph).get_hash().get(&coerced).expect("Retrieval should work").kind) {
                    move_cursor!($dir).move_global(&state).expect("Motion should succeed")
                } else {
                    move_cursor!($dir).move_local(&state).expect("Motion should succeed")
                };
                state.block_loc = dst.clone();
                let _ = state.coerce().expect("Post-motion coercion should succeed");
            )+
            assert_eq!(dst, addr!($($id),+));
        }};
        // Cycles are sequences of motions that end up at the leftmost root (i.e., <0, 0>)
        ($($dir:tt),+ _in_ $src:ident ||) => {{
            move_cursor!($($dir),+ _in_ $src -> <0, 0>);
        }};
        ($($dir:tt),+ _in_ $src:ident ##) => {{
            move_cursor!($($dir),+ _in_ $src -> <1, 0>);
        }};
        (U) => {$crate::digraph::state::CursorDir::UP};
        (D) => {$crate::digraph::state::CursorDir::DOWN};
        (L) => {$crate::digraph::state::CursorDir::LEFT};
        (R) => {$crate::digraph::state::CursorDir::RIGHT};
        (I) => {$crate::digraph::state::CursorDir::IN};
        (O) => {$crate::digraph::state::CursorDir::OUT};
    }

    const SOURCE: &'static str = "define f of x\noutput x\ndefine f1 of x\noutput x\ndone define\ndefine \
                          f2 of x\noutput x\ndefine f21 of x\noutput x\ndone define\ndefine f22 of \
                          x\noutput x\ndone define\ndone define\ndone define\ndefine g of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

    mod global {
        use super::*;

        #[test]
        fn global_invalid() {
            move_cursor!(D, D _in_ SOURCE -> InvalidMotion);
            move_cursor!(D, R, D, R, R _in_ SOURCE -> InvalidMotion);
            move_cursor!(D, R, D, R, D _in_ SOURCE -> InvalidMotion);
            move_cursor!(D, R, D, D _in_ SOURCE -> InvalidMotion);
        }

        #[test]
        fn global_compound() {
            move_cursor!(D, R _in_ SOURCE -> <0, 1, 1, 0>);
            move_cursor!(D, R, D, R _in_ SOURCE -> <0, 1, 1, 1, 1>);
            move_cursor!(D, R, D, R, L _in_ SOURCE -> <0, 1, 1, 1, 0>);
        }

        #[test]
        fn global_cycles() {
            move_cursor!(R, L _in_ SOURCE ||);
            move_cursor!(D, U _in_ SOURCE ||);
            move_cursor!(D, R, U _in_ SOURCE ||);
            move_cursor!(D, R, L, R, U _in_ SOURCE ||);
            move_cursor!(D, R, D, R, U, L, U _in_ SOURCE ||);
        }
    }

    mod local {
        use super::*;

        #[test]
        fn local_linear() {
            move_cursor!(R, I _in_ SOURCE -> <1, 0, 0>);
            move_cursor!(R, I, D _in_ SOURCE -> <1, 0, 1>);
            move_cursor!(R, I, D, D _in_ SOURCE -> <1, 0, 2>);
        }

        #[test]
        fn local_depth() {
            move_cursor!(R, I, D, D, I _in_ SOURCE -> <1, 0, 2, 0, 0>);
            move_cursor!(R, I, D, D, I, D _in_ SOURCE -> <1, 0, 2, 1, 0, 0>);
            move_cursor!(R, I, D, D, I, D, D _in_ SOURCE -> <1, 0, 2, 1, 0, 1>);
            move_cursor!(R, I, D, D, I, D, R _in_ SOURCE -> <1, 0, 2, 1, 1, 0>);
            move_cursor!(R, I, D, D, I, D, D, D _in_ SOURCE -> InvalidMotion);
        }

        #[test]
        fn local_cycles() {
            move_cursor!(R, I, D, U, O _in_ SOURCE ##);
            move_cursor!(R, I, D, D, U, U, O _in_ SOURCE ##);
            move_cursor!(R, I, D, D, U, U, O _in_ SOURCE ##);
            move_cursor!(R, I, D, D, O _in_ SOURCE ##);
        }
    }
}
