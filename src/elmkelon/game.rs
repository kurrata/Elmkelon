use elmkelon;

pub struct Game {
    pub world_map: elmkelon::map::world_map::WorldMap,
}

impl Game {
    pub fn create() -> Game {
        trace!("Creating game instance");
        let mut game = Game {
            world_map: elmkelon::map::world_map::WorldMap::create()
        };
        game.load_world_map();
        game
    }

    pub fn run(&self) {
        info!("Game is running");
    }

    fn load_world_map(&mut self) {
        self.world_map.grow(10);
    }
}