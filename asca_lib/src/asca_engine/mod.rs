use sdl2::{event::Event, keyboard::Keycode};

mod automata_engine;
mod render_engine;
mod tick_timer;
mod thread_manager;

use crate::sdl2_manager::SDLSystems;
use automata_engine::AutomataBoard;
use render_engine::Renderer;
use tick_timer::TickTimer;

#[derive(Default, PartialEq)]
enum EngineState {
    #[default]
    Paused,
    Running,
    Exit,
}

pub struct Engine {
    game_board: AutomataBoard,
    tick_timer: TickTimer,
    engine_state: EngineState,
    sdl_systems: SDLSystems,
    renderer: Renderer,
}

impl Default for Engine {
    fn default() -> Self {
        let mut sdl2_instance = SDLSystems::init();
        sdl2_instance.init_video();
        sdl2_instance.init_event_pump();

        const BOARD_SIZE_DEFAULT: usize = 2500;
        let mut board = AutomataBoard::init(BOARD_SIZE_DEFAULT, BOARD_SIZE_DEFAULT);
        board.seed_board(0.25);

        let renderer = Renderer::new(sdl2_instance.get_video_subsystem());

        Self {
            tick_timer: TickTimer::default(),
            engine_state: EngineState::Paused,
            game_board: board,
            sdl_systems: sdl2_instance,
            renderer,
        }
    }
}

impl Engine {
    pub fn init() -> Self {
        const BOARD_SIZE_DEFAULT: usize = 2500;
        let mut sdl2_instance = SDLSystems::init();
        sdl2_instance.init_video();
        sdl2_instance.init_event_pump();

        let mut board = AutomataBoard::init(BOARD_SIZE_DEFAULT, BOARD_SIZE_DEFAULT);
        board.seed_board(0.25);

        let renderer = Renderer::new(sdl2_instance.get_video_subsystem());

        Self {
            tick_timer: TickTimer::default(),
            engine_state: EngineState::Paused,
            game_board: board,
            sdl_systems: sdl2_instance,
            renderer,
        }
    }

    pub fn start(&mut self) -> &mut Self {
        //self.game_board.seed_board(0.25);
        self.engine_state = EngineState::Running;
        self
    }

    pub fn exit(&mut self) -> &mut Self {
        self.engine_state = EngineState::Exit;
        self
    }

    // TODO:
    // Implement a timer for rendering, fps
    //TODO:
    //Implement a tick timer for the game logic, tps
    //TODO:
    //Refactor game logic(tick) into it's own function/modlue
    pub fn run(&mut self) {
        'running: loop {
            //If the engine is currently paused, skip
            if self.engine_state == EngineState::Paused {
                return;
            }
            //If the engine is to exit, break the loop
            else if self.engine_state == EngineState::Exit {
                dbg!("Exiting engine!");
                break 'running;
            }
            //If it the engine is supposed to tick foreward, execute engine logic
            else if self.tick_timer.is_tick() {
                let board = &mut self.game_board;
                let renderer = &mut self.renderer;

                for event in self.sdl_systems.event_pump().poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => self.engine_state = EngineState::Exit, //std::process::exit(0),
                        _ => {}
                    }
                }

                board.advance();

                renderer.draw();
            }
        }
    }
}
