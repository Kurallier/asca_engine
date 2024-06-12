//TODO:
// [x] Establish camera
// Add camera controls
// Then add basic user event handling
// Add some basic ui
// Finally refactor the engine code
// Then we donzos
extern crate sdl2;

use conway_engine::ConwayEngine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2_manager::SDLSystems;
use std::time::Duration;

//My modules
mod conway_engine;
mod sdl2_manager;

pub fn main() {


    let mut conway_engine = ConwayEngine::init();


    //Hey Jackass, don't forget to set the engine in a running state next time
    conway_engine.play();
    'running: loop {


        /*
        conway_engine.calculate_surr_life();
        conway_engine.apply_con_rules();
        canvas
            .with_texture_canvas(&mut rendering_texture, |mut canvas| {
                canvas.clear();

                conway_engine.sdl2_draw_cells(&mut canvas);
            })
            .expect("L bozo");

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas
            .copy(&rendering_texture, cam_source, cam_final)
            .expect("Failed to copy texture");

        canvas.present();
        */
        conway_engine.advance();

        //Dirty way to limit ticks
        /*
        unsafe {
            SDL_Delay(256)
        }
        */
    }
}
