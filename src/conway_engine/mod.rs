mod con_board;
mod con_render;

use crate::sdl2_manager::SDLSystems;

use con_board::ConAutomataBoard;
use sdl2::{
    event::Event,
    keyboard::Keycode,
};

use self::con_render::ConwayRenderer;

#[derive(Default, PartialEq)]
enum EngineState {
    #[default]
    Paused,
    Running,
}

pub struct ConwayEngine {
    game_board: ConAutomataBoard,
    current_tick: usize,
    engine_state: EngineState,
    sdl_systems: SDLSystems,
    renderer: ConwayRenderer,
}

const BOARD_SIZE_DEFAULT: usize = 2500;

impl ConwayEngine {
    pub fn init() -> Self {
        let mut sdl2_instance = SDLSystems::init();
        sdl2_instance.init_video();
        sdl2_instance.init_event_pump();

        let mut board = ConAutomataBoard::init(BOARD_SIZE_DEFAULT, BOARD_SIZE_DEFAULT);
        board.seed_board(0.35);

        let renderer = ConwayRenderer::new(sdl2_instance.get_video_subsystem());

        Self {
            current_tick: 0,
            //HACK: Temporary
            engine_state: EngineState::Running,
            game_board: board,
            sdl_systems: sdl2_instance,
            renderer,
        }
    }

    pub fn play(&mut self) -> &mut Self {
        self.game_board.seed_board(0.25);
        self.engine_state = EngineState::Running;
        self
    }

    // TODO:
    // Implement a timer for rendering, fps
    //TODO:
    //Implement a tick timer for the game logic, tps
    //TODO:
    //Refactor game logic(tick) into it's own function/modlue
    pub fn advance(&mut self) {
        if self.engine_state == EngineState::Paused {
            return;
        } else {
            let board = &mut self.game_board;
            let renderer = &mut self.renderer;

            for event in self.sdl_systems.event_pump().poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => std::process::exit(0),
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => board.seed_board(0.25),
                    _ => {}
                }
            }

            // Where the tick implementation is important
            // --------------------------------
            /*
            board.calculate_surr_life();
            board.apply_con_rules();
            */
            // --------------------------------

            renderer.draw();

            self.current_tick += 1;
        }
    }
}
