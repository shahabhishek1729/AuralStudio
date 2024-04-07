use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub children: Vec<Node>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    kind: String,
    name: Option<Value>,
    line: usize,
}

pub fn parse() -> Vec<Node> {
    vec![
        Node {
            id: 1,
            children: vec![],
            blocks: vec![Block {
                kind: String::from("library"),
                name: Some(Value::Text(String::from("numpy"))),
                line: 1,
            }],
        },
        Node {
            id: 2,
            children: vec![
                Node {
                    id: 3,
                    children: vec![],
                    blocks: vec![Block {
                        kind: String::from("output"),
                        name: Some(Value::Text(String::from("hello"))),
                        line: 1,
                    }],
                },
                Node {
                    id: 4,
                    children: vec![],
                    blocks: vec![Block {
                        kind: String::from("output"),
                        name: Some(Value::Text(String::from("bye"))),
                        line: 1,
                    }],
                },
            ],
            blocks: vec![
                Block {
                    kind: String::from("function"),
                    name: Some(Value::Text(String::from("main(int argc, char **argc"))),
                    line: 1,
                },
                Block {
                    kind: String::from("output"),
                    name: Some(Value::Text(String::from("hello + 42"))),
                    line: 2,
                },
                Block {
                    kind: String::from("variable"),
                    name: Some(Value::Text(String::from("hello"))),
                    line: 3,
                },
                Block {
                    kind: String::from("arrow"),
                    name: Some(Value::Text(String::from("->"))),
                    line: 3,
                },
                Block {
                    kind: String::from("constant"),
                    name: Some(Value::Number(6)),
                    line: 3,
                },
                Block {
                    kind: String::from("operator"),
                    name: Some(Value::Text(String::from("+"))),
                    line: 3,
                },
                Block {
                    kind: String::from("constant"),
                    name: Some(Value::Number(43)),
                    line: 3,
                },
            ],
        },
    ]
}
