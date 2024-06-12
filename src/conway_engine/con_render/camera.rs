use sdl2::rect::Rect;


//TODO: May need to add setter and getter functions
pub struct Camera {
    cam_source: Rect,
    cam_present: Rect,
    cam_x: isize,
    cam_y: isize,
    cam_w: u32,
    cam_h: u32,
    cam_zoom: u32,
}

impl Camera {
    pub fn new(
        camera_width: u32,
        camera_height: u32,
        start_x: isize,
        start_y: isize,
        camera_zoom: u32,
    ) -> Self {
        let cam_s = Rect::new(
            start_x as i32,
            start_y as i32,
            camera_width / camera_zoom,
            camera_height / camera_zoom,
        );

        let cam_f = Rect::new(5, 5, camera_width - 10, camera_height - 10);

        Self {
            cam_source: cam_s,
            cam_present: cam_f,
            cam_x: start_x,
            cam_y: start_y,
            cam_w: camera_width,
            cam_h: camera_height,
            cam_zoom: camera_zoom,
        }
    }

    pub fn source_pixels(&self) -> &Rect {
        &self.cam_source
    }

    pub fn cam_present(&self) -> &Rect {
        &self.cam_present
    }
}
