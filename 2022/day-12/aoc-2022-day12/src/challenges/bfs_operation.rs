use super::coordinates::Coordinates;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct BFSOperation {
    pub node: Coordinates,
    pub level: u32,
}

pub fn new_bfs_operation(node: Coordinates, level: u32) -> BFSOperation {
    BFSOperation { node, level }
}
