use std::collections::HashMap;
use elmkelon::map::coordinate::Coordinate;
use elmkelon::map::cell::Cell;
use elmkelon::map::terrain;
use std::cmp;

pub struct WorldMap {
    pub map: HashMap<Coordinate, Cell>,
}

impl WorldMap {
    pub fn create() -> WorldMap {
        trace!("Creating map instance");
        let mut world_map: WorldMap = WorldMap { map: HashMap::new() };
        let root_coordinate: Coordinate = Coordinate::create(0, 0);
        world_map.add_cell(root_coordinate);
        world_map
    }

    pub fn grow(&mut self, radius: i32) {
        trace!("Growing map");
        for x in -radius..radius + 1 {
            let r1 = cmp::max(-radius, -x - radius);
            let r2 = cmp::min(radius, -x + radius);
            for r in r1..r2 + 1 {
                self.add_cell(Coordinate::create(x, -x - r))
            }
        }
        trace!("Map has grown to {} cells", self.map.len())
    }

    pub fn get_radius(&self) -> i32 {
        let mut radius: i32 = 0;
        let mut done: bool = false;
        while !done {
            let current: Coordinate = Coordinate::create(radius, 0);
            let found = self.map.contains_key(&current);
            if found {
                radius = radius + 1;
            } else {
                done = true;
            }
        }
        radius - 1
    }

    fn add_cell(&mut self, coordinate: Coordinate) {
        let cell: Cell = Cell::create(&coordinate, terrain::TERRAIN_OCEAN);
        self.map.insert(coordinate, cell);
    }
}
