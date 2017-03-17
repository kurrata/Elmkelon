use elmkelon;

pub struct Game {
    pub world_map: elmkelon::map::world_map::WorldMap,
    pub config: elmkelon::config::Config,
    pub netwrok: elmkelon::network::network::Network,
}

impl Game {
    pub fn create() -> Game {
        trace!("Creating game instance");
        let game = Game {
            world_map: elmkelon::map::world_map::WorldMap::create(),
            config: elmkelon::config::Config::create(),
            netwrok: elmkelon::network::network::Network::create(),
        };
        game
    }

    pub fn run(&mut self) {
        self.load_world_map();
        self.netwrok.listen();
        info!("Game is running");
    }

    fn load_world_map(&mut self) {
        self.world_map.grow(self.config.get_map_radius());
    }
}