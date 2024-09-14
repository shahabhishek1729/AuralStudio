use crate::digraph::parser::{Node, NodeKind};
use crate::digraph::util::HORIZ_CHILDREN;
use crate::prelude::CursorError;
use serde;
use std::collections::HashMap;

#[macro_export]
macro_rules! addr {
    ($($id:expr),*) => {
        crate::digraph::address::Address::new(vec![$($id),*])
    };
}

/// Represents a single address within the digraph.
/// An address is of the form [hpos, vpos, hpos, vpos, ...], where `hpos` denotes a horizontal
/// position, and `vpos` denotes a vertical position.
///
/// # Examples
/// ```rust
/// // Denotes (from the root) the first child, second level, second child, and second level.
/// let address = Address::new(vec![0, 1, 1, 1]);
/// assert_eq!(address.up(), true);
/// assert_eq!(*address, [0, 1, 1, 0]);
/// assert_eq!(address.down(), true);
/// assert_eq!(*address, [0, 1, 1, 1]);
/// // From this address only vertical motions are possible.
/// assert_eq!(address.left(), false);
/// assert_eq!(address.right(), false);
/// ```
///
/// ```rust
/// // Denotes (from the root) the first child, second level, and second child.
/// let address = Address::new(vec![0, 1, 1]);
/// assert_eq!(address.left(), true);
/// assert_eq!(*address, [0, 1, 0]);
/// assert_eq!(address.right(), true);
/// assert_eq!(*address, [0, 1, 1]);
/// // From this address only horizontal motions are possible.
/// assert_eq!(address.up(), false);
/// assert_eq!(address.down(), false);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Address {
    pub addr: Vec<usize>,
}

impl Address {
    pub(super) fn new(addr: Vec<usize>) -> Self {
        Self { addr }
    }

    fn from_ref(addr: &Vec<usize>) -> Self {
        Self { addr: addr.clone() }
    }

    /// Coerces a given subtree's address to point to the node at the subtree's root.
    /// In effect, appends as many zeros as necessary to the end of an address until the address
    /// points to a valid node.
    pub(crate) fn coerce(&self, graph: &HashMap<Address, &Node>) -> Result<Self, CursorError> {
        let mut i = 0;
        let max_iters = graph.len();
        // Keep pushing zeroes until we get to a valid address.
        let mut addr = (*self.clone()).clone();
        while !graph.contains_key(&Address::from_ref(&addr)) {
            // FIXME: Is this the best way to determine if addresses exist in the graph or not?
            if i > max_iters {
                return Err(CursorError::AddrNotFound(self.clone()));
            }
            addr.push(0);
            i += 1;
        }
        Ok(Address::new(addr))
    }

    /// Increment a given address (e.g., <0, 0, 0> -> <0, 0, 1>). Does not check validity.
    pub(super) fn next(&self) -> Option<Address> {
        let mut new_addr = self.addr.clone();
        let last = new_addr.last_mut()?;
        *last += 1;
        Some(Address::new(new_addr))
    }

    /// Append to a given address (e.g., <0, 0> -> <0, 0, 1>). Does not check validity.
    pub(super) fn join(&self, to_join: &[usize]) -> Address {
        let mut new_addr = self.addr.clone();
        new_addr.extend_from_slice(to_join);
        Address::new(new_addr)
    }
}

/// Formats an address in IPv4-style (e.g. vec![1, 2, 3, 4] -> "1.2.3.4")
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (*self.clone())
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

/// Allows us to get the internal address vector from an `Address` object.
impl std::ops::Deref for Address {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.addr
    }
}

/// Serializes addresses to be easier to work with in JSON (turns <1, 2, 3, 4> into 1.2.3.4)
impl serde::ser::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let addrs = self.addr.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        serializer.serialize_str(&addrs.join("."))
    }
}

/// Reverses serialization (turns 1.2.3.4 into <1, 2, 3, 4>)
impl<'de> serde::de::Deserialize<'de> for Address {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let addr = s
            .split('.')
            .map(|s| {
                s.parse::<usize>()
                    .expect(&format!("Address should be numbers, but got {s}"))
            })
            .collect::<Vec<_>>();
        Ok(Self::new(addr))
    }
}

pub(crate) trait Addressable {
    fn get_hash(&self) -> HashMap<Address, &Node> {
        unreachable!("Method `get_hash` should only be called on &[Node] and &'mut [Node]")
    }

    /// There are 4 rules to addressing nodes:
    /// 1. The roots have address (i, 0, 0) and their direct children will be (i, 0, j)
    /// 2. If a node has vertical children, append a 0. For horizontal children, add two 0's.
    /// 3. Vertical children increment the child (i.e., last) index; e.g., (i, 0, j, k).
    /// 4. Horizontal children increment the child second-to-last index by 1, and the last index
    ///    by k; e.g., (i, 0, j, 1, k). (+1 = on the lower level, +k = kth child on that level).
    fn fill_addr(&mut self) {
        unreachable!("Method `fill_addr` should never be called on &[Node]")
    }
}

impl Addressable for Vec<Node> {
    fn get_hash(&self) -> HashMap<Address, &Node> {
        let mut hm: HashMap<Address, &Node> = HashMap::new();
        // Recursively traverse the graph, adding each node to the map and handling its subtree.
        fn _inner<'a>(node: &'a Node, _hm: &mut HashMap<Address, &'a Node>) {
            _hm.insert(node.addr.clone(), node);
            node.children.iter().for_each(|n| _inner(n, _hm));
        }
        self.iter().for_each(|ref n| _inner(n, &mut hm));
        hm
    }
}

impl Addressable for &mut [Node] {
    fn fill_addr(&mut self) {
        fn _inner(node: &mut Node, i: isize, parent_addr: &Address, horiz_: bool) {
            let mut addr: Vec<usize> = (*parent_addr.clone()).clone();
            let horiz = HORIZ_CHILDREN.contains(&node.kind) && horiz_;
            if horiz {
                let last_idx = addr.len() - 2;
                // Increment second-to-last since these children are on the level below the parent.
                addr[last_idx] += 1;
                // Increment last for each child's distinct horizontal position within that level.
                addr[last_idx + 1] += i as usize;
            } else {
                let last_idx = addr.len() - 1;
                addr[last_idx] += (1 + i) as usize;
            }

            // For FNDEF, CONDTL and other nodes with horizontal children, the first 0 pushed
            // references the vertical "level", and the 2nd 0 references the node's horizontal
            // position within that level.
            if !node.children.is_empty() {
                addr.push(0);
                if node.has_subtree() && horiz_ {
                    addr.push(0);
                }
            }
            node.addr = Address::new(addr);
            node.parent_addr = parent_addr.clone();

            let mut fn_idx: isize = -1;
            node.children.iter_mut().enumerate().for_each(|(i_, n_)| {
                match n_.kind {
                    NodeKind::FNDEF => {
                        fn_idx += 1;
                        _inner(n_, fn_idx as isize, &node.addr, true);
                    }
                    _ => _inner(n_, i_ as isize, &node.addr, true),
                };
            });
        }

        for (i, ref mut node) in self.into_iter().enumerate() {
            // i is set to -1 here because we do not want to change the addresses of the roots.
            _inner(node, -1, &addr![i, 0], false);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{digraph::address::Addressable, Node, Parser};

    #[macro_use]
    mod addresser {
        use super::*;
        use crate::digraph::parser::{NodeKind::*, OpKind::*, Piece::*};
        use crate::{make_node, piece};

        #[test]
        fn condtl_no_subfn() {
            let source = "define f of x\noutput x\nlet my_age be 3\ndone define\ndefine g of x\n\
                          output string hi done\ndone define\ndefine h of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

            let mut parser = Parser::new(String::from(source)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();

            assert_eq!(
                nodes,
                vec![
                    make_node!(L 1 @ 0,0,0 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                        make_node!(L 2 @ 0,0,1 -> OUTPUT [piece!(IDENT "x")]),
                        make_node!(L 3 @ 0,0,2 -> VARDECL [piece!(IDENT "my_age"), OP(ASSN), piece!(# 3)])
                    }),
                    make_node!(L 5 @ 1,0,0 -> FNDEF [piece!(IDENT "g"), piece!(IDENT "x")]; {
                        make_node!(L 6 @ 1,0,1 -> OUTPUT [piece!(TEXT "hi")])
                    }),
                    make_node!(L 8 @ 2,0,0 -> FNDEF [piece!(IDENT "h"), piece!(IDENT "x")]; {
                        make_node!(L 9 @ 2,0,1 -> OUTPUT [piece!(IDENT "x"), OP(ADD), piece!(# 1)]),
                        make_node!(L 10 @ 2,0,2,0,0 -> CONDTL [piece!(IDENT "x"), OP(EQ), piece!(# 3)]; {
                            make_node!(L 11 @ 2,0,2,1,0,0 -> CONDTLY []; {
                                make_node!(L 12 @ 2,0,2,1,0,1 -> OUTPUT [piece!(IDENT "x")])
                            }),
                            make_node!(L 13 @ 2,0,2,1,1,0 -> CONDTLN []; {
                                make_node!(L 14 @ 2,0,2,1,1,1 -> OUTPUT [piece!(IDENT "y")])
                            })
                        })
                    }),
                ]
            );
        }

        #[test]
        fn condtl_and_subfn() {
            let source = "define f of x\noutput x\ndefine f1 of x\noutput x\ndone define\ndefine \
                          f2 of x\noutput x\ndefine f21 of x\noutput x\ndone define\ndefine f22 of \
                          x\noutput x\ndone define\ndone define\ndone define\ndefine h of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

            let mut parser = Parser::new(String::from(source)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();

            assert_eq!(
                nodes,
                vec![
                    make_node!(L 1 @ 0,0,0 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                        make_node!(L 2 @ 0,0,1 -> OUTPUT [piece!(IDENT "x")]),
                        make_node!(L 3 @ 0,1,0,0 -> FNDEF [piece!(IDENT "f1"), piece!(IDENT "x")]; {
                            make_node!(L 4 @ 0,1,0,1 -> OUTPUT [piece!(IDENT "x")])
                        }),
                        make_node!(L 6 @ 0,1,1,0,0 -> FNDEF [piece!(IDENT "f2"), piece!(IDENT "x")]; {
                            make_node!(L 7 @ 0,1,1,0,1 -> OUTPUT [piece!(IDENT "x")]),
                            make_node!(L 8 @ 0,1,1,1,0,0 -> FNDEF [piece!(IDENT "f21"), piece!(IDENT "x")]; {
                                make_node!(L 9 @ 0,1,1,1,0,1 -> OUTPUT [piece!(IDENT "x")])
                            }),
                            make_node!(L 11 @ 0,1,1,1,1,0 -> FNDEF [piece!(IDENT "f22"), piece!(IDENT "x")]; {
                                make_node!(L 12 @ 0,1,1,1,1,1 -> OUTPUT [piece!(IDENT "x")])
                            })
                        })
                    }),
                    make_node!(L 16 @ 1,0,0 -> FNDEF [piece!(IDENT "h"), piece!(IDENT "x")]; {
                        make_node!(L 17 @ 1,0,1 -> OUTPUT [piece!(IDENT "x"), OP(ADD), piece!(# 1)]),
                        make_node!(L 18 @ 1,0,2,0,0 -> CONDTL [piece!(IDENT "x"), OP(EQ), piece!(# 3)]; {
                            make_node!(L 19 @ 1,0,2,1,0,0 -> CONDTLY []; {
                                make_node!(L 20 @ 1,0,2,1,0,1 -> OUTPUT [piece!(IDENT "x")])
                            }),
                            make_node!(L 21 @ 1,0,2,1,1,0 -> CONDTLN []; {
                                make_node!(L 22 @ 1,0,2,1,1,1 -> OUTPUT [piece!(IDENT "y")])
                            })
                        })
                    }),
                ]
            );
        }
    }

    mod addr_ops {
        use super::*;
        use crate::digraph::address::Addressable;

        const SOURCE: &'static str =
            "define f of x\noutput x\nlet my_age be 3\ndone define\ndefine g of x\n\
                          output string hi done\ndone define\ndefine h of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

        fn _test_setup() -> Vec<Node> {
            let mut parser = Parser::new(String::from(SOURCE)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();
            nodes
        }

        #[test]
        fn basic_coercion() {
            let nodes = _test_setup();
            let addr = addr!(0, 0);
            assert_eq!(
                addr.coerce(&nodes.get_hash())
                    .expect("Coercion should work"),
                addr!(0, 0, 0)
            );
        }

        #[test]
        #[should_panic]
        fn invalid_coercion() {
            let nodes = _test_setup();
            let addr = addr!(3, 0);
            addr.coerce(&nodes.get_hash())
                .expect("Coercion should work");
        }

        #[test]
        fn next_addr() {
            let addr = addr!(0, 0, 0);
            assert_eq!(addr.next(), Some(addr!(0, 0, 1)));
        }
    }
}
