use sdl2::{render::Canvas, video::Window, VideoSubsystem};

pub struct WindowData {
    window_w: u32,
    window_h: u32,
    windowing_context: Option<Window>,
    canvas_context: Option<Canvas<Window>>
}
// This will create a window for the app with default settings
// Window Width (In pixels) will be 1920
// Window Height (In pixels) will be 1080
impl WindowData {
    pub fn new_with_default(video_sys: &VideoSubsystem) -> WindowData {
        let window_w = 1280;
        let window_h = 720;
        let window = video_sys
            .window("ASCA Rewrite", window_w, window_h)
            //.hidden()
            //.position_centered()
            .resizable()
            //.fullscreen_desktop()
            .build()
            .expect("Failed to create window!");
        let canvas = window.into_canvas().build().unwrap();

        WindowData {
            window_w,
            window_h,
            windowing_context: None,
            canvas_context: Some(canvas),
        }
    }

    /*
    pub fn build_canvas(&mut self) ->  Canvas<Window> {
        let window = self.windowing_context.as_mut().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        canvas

    }
    */

    pub fn canvas(&self) -> &Canvas<Window> {
        self.canvas_context.as_ref().unwrap()
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        self.canvas_context.as_mut().unwrap()
    }
    pub fn window_width(&self) -> u32 {
        self.window_w
    }

    pub fn window_height(&self) -> u32 {
        self.window_h
    }

}
