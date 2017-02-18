#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn create(x: i32, y: i32) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}
