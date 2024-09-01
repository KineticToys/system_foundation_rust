#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraphError {
    NoSuchNode,
    NoSuchEdge,
    DuplicateNodeId,
    DuplicateEdgeId,
}