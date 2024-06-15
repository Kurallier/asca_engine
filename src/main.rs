//TODO:
// [x] Establish camera
// Add camera controls
// Then add basic user event handling
// Add some basic ui
// Finally refactor the engine code
// Then we donzos

use conway_engine::ConwayEngine;

//My modules
mod conway_engine;
mod sdl2_manager;

pub fn main() {


    let mut conway_engine = ConwayEngine::init();


    //Hey Jackass, don't forget to set the engine in a running state next time
    conway_engine.play();
    'running: loop {
        conway_engine.advance();
    }
}
