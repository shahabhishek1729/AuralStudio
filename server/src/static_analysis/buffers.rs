use crate::digraph::address::Address;
use crate::digraph::parser::{Node, PieceIdx};

pub(crate) struct Buffer<'a> {
    len: usize,
    buf: Vec<Action<'a>>,
}

pub(crate) enum Action<'p> {
    InsertToken(Address),
    InsertVal(Address, PieceIdx<'p>),
    InsertOp(Address, PieceIdx<'p>),
    DeleteNode(Node),
}

impl Action<'_> {
    fn invert(&self) -> Self {
        todo!()
    }
}
