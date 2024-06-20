use sdl2::{
    keyboard::KeyboardUtil, mouse::MouseUtil, rect::Rect, AudioSubsystem, EventPump,
    EventSubsystem, GameControllerSubsystem, HapticSubsystem, JoystickSubsystem, Sdl,
    SensorSubsystem, TimerSubsystem, VideoSubsystem,
};


pub struct SDLSystems {
    sdl_context: Sdl,
    audio: Option<AudioSubsystem>,
    event: Option<EventSubsystem>,
    event_pump: Option<EventPump>,
    game_controller: Option<GameControllerSubsystem>,
    haptic: Option<HapticSubsystem>,
    joystick: Option<JoystickSubsystem>,
    keyboard: Option<KeyboardUtil>,
    mouse: Option<MouseUtil>,
    sensor: Option<SensorSubsystem>,
    timer: Option<TimerSubsystem>,
    //TODO: Find a way to make this member private
    pub video: Option<VideoSubsystem>,
}

impl SDLSystems {
    pub fn init() -> Self {
        let sdl = sdl2::init().unwrap();

        let new_self = Self {
            sdl_context: sdl,
            audio: None,
            event: None,
            event_pump: None,
            game_controller: None,
            haptic: None,
            joystick: None,
            keyboard: None,
            mouse: None,
            sensor: None,
            timer: None,
            video: None,
        };
        new_self
    }
}

//Getter functions
impl SDLSystems {
    //TODO: Make this function have an err case
    pub fn get_video_subsystem(&self) -> &VideoSubsystem {
        self.video.as_ref().unwrap()
    }
    pub fn event_pump(&mut self) -> &mut EventPump {
        self.event_pump.as_mut().unwrap()
    }
}

//This block will hold the init functions
impl SDLSystems {
    pub fn init_video(&mut self) {
        match &self.video {
            Some(_) => return,
            None => {
                let video_sys = self
                    .sdl_context
                    .video()
                    .expect("Failed to init the SDL2 Video Subsystem");
                self.video = Some(video_sys);
            }
        }
    }

    pub fn init_event_pump(&mut self) {
        match self.event_pump {
            Some(_) => {
                return;
            }
            None => {
                self.event_pump = Some(
                    self.sdl_context
                        .event_pump()
                        .expect("Failed to create event pump"),
                );
            }
        }
    }
}
