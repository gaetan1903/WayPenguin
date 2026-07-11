use waypenguin_backends::{BackendError, DesktopBackend, DesktopWindow, ScreenInfo};

pub struct GnomeBackend;

impl DesktopBackend for GnomeBackend {
    fn get_screens(&self) -> Vec<ScreenInfo> {
        vec![]
    }

    fn get_cursor_position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn get_last_click(&self) -> Option<(i32, i32)> {
        None
    }

    fn clear_last_click(&mut self) {}

    fn create_window(
        &mut self,
        _width: u32,
        _height: u32,
        _x: i32,
        _y: i32,
    ) -> Result<Box<dyn DesktopWindow>, BackendError> {
        Err(BackendError::Unsupported)
    }
}
