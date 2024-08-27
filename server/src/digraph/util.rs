use crate::digraph::parser::NodeKind;
use crate::Node;
use phf::phf_map;

/// The number of children each kind of parent can have (most can only have 1, but conditionals
/// have 2, for a "yes" and a "no" branch.
pub(super) static N_ROOT_CHILDREN: phf::Map<&'static str, u8> = phf_map! {
    "CONDTL" => 2,
    "CONDTLY" => 1,
    "CONDTLN" => 1,
    "WHLLOOP" => 1,
    "FORLOOP" => 1,
    "FNDEF" => 1,
};

/// The kinds of global block nodes (currently only functions).
pub(super) const GLOBAL_BLOCKS: &[NodeKind] = &[NodeKind::FNDEF];

/// Finds the child nodes of a node that are themselves global blocks (e.g., functions).
pub(super) fn _filter_children(node: &Node) -> impl Iterator<Item = &Node> {
    node.children
        .iter()
        .filter(|c| !GLOBAL_BLOCKS.contains(&c.kind))
}

/// There are 3 kinds of nodes which can be drawn with an arrow from a parent node
pub(super) const HORIZ_CHILDREN: &[NodeKind; 3] =
    &[NodeKind::FNDEF, NodeKind::CONDTLY, NodeKind::CONDTLN];
