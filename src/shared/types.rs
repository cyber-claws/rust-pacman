use crate::shared::enums::Direction;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub direction_from_source: Direction,
}
