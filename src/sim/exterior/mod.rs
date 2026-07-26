pub mod path;
pub mod placement;

pub use path::{advance_along_path, advance_along_waypoints, EXTERIOR_WAYPOINTS};
pub use placement::{is_buildable_cell, try_place_tower, PlaceError, CELL_SIZE};
