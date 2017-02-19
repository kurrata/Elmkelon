use elmkelon;

pub struct Game {
    pub world_map: elmkelon::map::world_map::WorldMap,
    pub config: elmkelon::config::Config
}

impl Game {
    pub fn create() -> Game {
        trace!("Creating game instance");
        let mut game = Game {
            world_map: elmkelon::map::world_map::WorldMap::create(),
            config: elmkelon::config::Config::create(),
        };
        game.load_world_map();
        game
    }

    pub fn run(&self) {
        info!("Game is running");
    }

    fn load_world_map(&mut self) {
        self.world_map.grow(self.config.get_map_radius());
        println!("Size is {}", self.world_map.map.len());
    }
}