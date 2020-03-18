//! Computes distances to polygon edges and vertices and can check whether points are inside/outside.

mod io;
mod point;
mod tree;

pub use crate::io::read_vector;
pub use crate::point::Point;
pub use crate::tree::build_tree;
pub use crate::tree::contains_points;
pub use crate::tree::create_polygon;
pub use crate::tree::distances_nearest_edges;
pub use crate::tree::nearest_vertices;
pub use crate::tree::Edge;
pub use crate::tree::IndexPoint;
pub use crate::tree::Node;