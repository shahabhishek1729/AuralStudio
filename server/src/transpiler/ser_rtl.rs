use crate::digraph::parser::Node;
use crate::prelude::*;

pub(crate) trait Transpiler {
    fn transpile(graph: impl IntoIterator<Item = Node>) -> Result<String, TranspileError>;
}

pub(crate) struct RTLTranspiler;
impl Transpiler for RTLTranspiler {
    fn transpile(graph: impl IntoIterator<Item = Node>) -> Result<String, TranspileError> {
        let mut rtl = String::new();

        fn _inner(node: &Node, rtl: &mut String) -> Result<(), TranspileError> {
            let curr_rtl = node.rtl.clone().unwrap_or("".into());
            rtl.push_str(&format!("{}\n", curr_rtl));

            let mut iter = false;
            let block = curr_rtl.split(" ").next().unwrap_or("");

            for child in node.children.iter() {
                _inner(child, rtl)?;
                iter = true;
            }

            if iter && !block.is_empty() {
                rtl.push_str(&format!("done {}\n", block));
            }
            Ok(())
        }

        for ref node in graph.into_iter() {
            _inner(node, &mut rtl)?;
        }

        Ok(rtl)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digraph::parser::Parser;

    #[test]
    fn transpiler() {
        let source = "define f of x\noutput x\nlet my_age be 3\ndone define\ndefine g of x\n\
                          output string hi done\ndone define\ndefine h of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define\n";
        let mut parser = Parser::new(source.into()).unwrap();
        let tokens = parser.parse().unwrap();

        assert_eq!(
            RTLTranspiler::transpile(tokens)
                .expect("Transpilation should work")
                .replace("\n\n", "\n"),
            source
        );
    }

    #[test]
    fn transpiler2() {
        let source = "define f of x\noutput x\ndone define\n";

        let mut parser = Parser::new(source.into()).unwrap();
        let tokens = parser.parse().unwrap();

        assert_eq!(
            RTLTranspiler::transpile(tokens)
                .expect("Transpilation should work")
                .replace("\n\n", "\n"),
            source
        );
    }
}
