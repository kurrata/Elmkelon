use elmkelon::map::coordinate::Coordinate;

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub terrain: u8,
}

impl Cell {
    pub fn create(coordinate: &Coordinate, terrain: u8) -> Cell {
        Cell { x: coordinate.x, y: coordinate.y, terrain: terrain }
    }

    //Hex axial neighbor coordinates
    pub fn neighbors(&self) -> [Coordinate; 6] {
        let neighbors: [Coordinate; 6] = [
            Coordinate::create(self.x + 1, self.y), //east
            Coordinate::create(self.x + 1, self.y - 1), //north east
            Coordinate::create(self.x, self.y - 1), //north west
            Coordinate::create(self.x - 1, self.y), //west
            Coordinate::create(self.x - 1, self.y + 1), //south west
            Coordinate::create(self.x, self.y + 1), //south east
        ];
        neighbors
    }
}