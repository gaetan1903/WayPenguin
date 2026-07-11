#[derive(Debug, Clone, PartialEq)]
pub struct ScreenInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendError {
    InitializationFailed(String),
    WindowCreationFailed(String),
    WindowError(String),
    Unsupported,
}

pub trait DesktopBackend {
    /// Return list of connected screens
    fn get_screens(&self) -> Vec<ScreenInfo>;

    /// Return the cursor's current absolute position on the virtual screen (x, y)
    fn get_cursor_position(&self) -> (i32, i32);

    /// Return the position of the last click on any pet window, if any
    fn get_last_click(&self) -> Option<(i32, i32)>;

    /// Clear the last click so it isn't processed again
    fn clear_last_click(&mut self);

    /// Return known window geometries for window-walking (stub for now)
    fn get_window_geometries(&self) -> Vec<WindowGeometry> {
        Vec::new()
    }

    /// Create a new window on this backend
    fn create_window(
        &mut self,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
    ) -> Result<Box<dyn DesktopWindow>, BackendError>;
}

pub trait DesktopWindow {
    /// Update the window dimensions
    fn set_size(&mut self, width: u32, height: u32) -> Result<(), BackendError>;

    /// Update the window's position on the screen
    fn set_position(&mut self, x: i32, y: i32) -> Result<(), BackendError>;

    /// Write a pixel buffer (ARGB8888) to the window
    fn present_pixels(&mut self, pixels: &[u32]) -> Result<(), BackendError>;

    /// Enable or disable click-through (input region handling)
    fn set_click_through(&mut self, click_through: bool) -> Result<(), BackendError>;
}
