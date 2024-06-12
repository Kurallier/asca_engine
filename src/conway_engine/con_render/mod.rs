use core::panic;
use std::{array, collections::HashMap, default};

use sdl2::{
    gfx::framerate::FPSManager, pixels::Color, render::{Canvas, Texture}, sys::SDL_Delay, version::Version, video::Window, VideoSubsystem
};

use self::{camera::Camera, window_handler::WindowData};

use super::ConwayEngine;
mod camera;
mod window_handler;

#[derive(PartialEq, Eq, Debug)]
enum RenderTextureType {
    Background,
    GameScene,
    Foreground,
    UI,
}

struct RenderTexture {
    texture: Texture,
    texture_type: RenderTextureType,
}

struct FrameRateManager {
    frame_counter: u64
}
impl FrameRateManager {
   fn new() -> Self {
        Self {
            frame_counter: 0
        }
    }
}

pub struct ConwayRenderer {
    window_data: WindowData,
    fps_mng: FrameRateManager,
    //TODO: Determine a good data structure for the textures
    background_texture: RenderTexture,
    game_scene: RenderTexture,
    foreground_texture: RenderTexture,
    ui_texture: RenderTexture,
    camera: Camera,
}

impl ConwayRenderer {
    pub fn new(sdl_video_sys: &VideoSubsystem) -> Self {
        let mut fps_mng = FrameRateManager::new();

        let window_data = WindowData::new_with_default(sdl_video_sys);
        let canvas = window_data.canvas();

        let texture_creator = canvas.texture_creator();
        let mut background_texture = texture_creator
            .create_texture_target(
                None,
                window_data.window_width(),
                window_data.window_height(),
            )
            .unwrap();
        let mut foreground_texture = texture_creator
            .create_texture_target(
                None,
                window_data.window_width(),
                window_data.window_height(),
            )
            .unwrap();
        let mut game_scene = texture_creator
            .create_texture_target(
                None,
                window_data.window_width(),
                window_data.window_height(),
            )
            .unwrap();
        let mut ui = texture_creator
            .create_texture_target(
                None,
                window_data.window_width(),
                window_data.window_height(),
            )
            .unwrap();

        let ren_text_bg = RenderTexture {
            texture: background_texture,
            texture_type: RenderTextureType::Background,
        };
        let ren_text_fg = RenderTexture {
            texture: foreground_texture,
            texture_type: RenderTextureType::Foreground,
        };
        let ren_text_gs = RenderTexture {
            texture: game_scene,
            texture_type: RenderTextureType::GameScene,
        };
        let ren_text_ui = RenderTexture {
            texture: ui,
            texture_type: RenderTextureType::UI,
        };

        let camera = Camera::new(
            window_data.window_width(),
            window_data.window_height(),
            (window_data.window_width() / 2) as isize,
            (window_data.window_height() / 2) as isize,
            3,
        );

        Self {
            window_data,
            fps_mng,
            background_texture: ren_text_bg,
            game_scene: ren_text_gs,
            foreground_texture: ren_text_fg,
            ui_texture: ren_text_ui,
            camera,
        }
    }

    //TODO: Pass over the pixel data to this funciton
    pub fn draw(&mut self) -> &mut Self {
        //TODO:
        //Figure out how to delay the drawing 
        
        //::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));

        let canvas = self.window_data.canvas_mut();

        let background_texture_data = &mut self.background_texture;
        let game_scene_texture_data = &mut self.game_scene;
        let foregound_texture_data = &mut self.foreground_texture;
        let ui_texture_data = &mut self.ui_texture;

        let texture_vec: Vec<(&mut Texture, &RenderTextureType)> = vec![
            (
                &mut background_texture_data.texture,
                &background_texture_data.texture_type,
            ),
            (
                &mut game_scene_texture_data.texture,
                &game_scene_texture_data.texture_type,
            ),
            (
                &mut foregound_texture_data.texture,
                &foregound_texture_data.texture_type,
            ),
            (&mut ui_texture_data.texture, &ui_texture_data.texture_type),
        ];

        canvas
            .with_multiple_texture_canvas(texture_vec.iter(), |texture_canvas, texture_type| {
                match *texture_type {
                    RenderTextureType::Background => {
                        draw_to_background(texture_canvas);
                    }
                    RenderTextureType::GameScene => {
                        draw_to_game_scene(texture_canvas);
                    }
                    RenderTextureType::Foreground => {
                        draw_to_foreground(texture_canvas);
                    }
                    RenderTextureType::UI => {
                        draw_to_ui(texture_canvas);
                    }
                    _ => panic!("Texture has no type!"),
                }
            })
            .unwrap();

        texture_vec.iter().for_each(|x| {
            canvas.copy(x.0, None, None).unwrap();
        });

        canvas.present();
        self
    }
}

fn draw_to_background(texture_canvas: &mut Canvas<Window>) {
    texture_canvas.set_draw_color(Color::GREY);
    texture_canvas.clear();
}

fn draw_to_game_scene(texture_canvas: &mut Canvas<Window>) {
    texture_canvas.set_draw_color(Color::GREY);
    texture_canvas.clear();
}
fn draw_to_foreground(texture_canvas: &mut Canvas<Window>) {
    texture_canvas.set_draw_color(Color::GREY);
    texture_canvas.clear();
}
fn draw_to_ui(texture_canvas: &mut Canvas<Window>) {
    texture_canvas.set_draw_color(Color::GREY);
    texture_canvas.clear();
}
