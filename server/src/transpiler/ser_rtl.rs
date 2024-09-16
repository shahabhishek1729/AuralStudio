use crate::digraph::parser::Node;
use crate::prelude::*;

trait Transpiler {
    fn transpile(graph: &[Node]) -> Result<String, TranspileError>;
}

struct RTLTranspiler;
impl Transpiler for RTLTranspiler {
    fn transpile(graph: &[Node]) -> Result<String, TranspileError> {
        todo!()
    }
}
