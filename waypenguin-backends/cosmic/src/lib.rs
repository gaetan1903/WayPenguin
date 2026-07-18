use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_layer, delegate_output, delegate_pointer, delegate_registry,
    delegate_relative_pointer, delegate_seat, delegate_shm,
    globals::GlobalData,
    output::OutputHandler,
    registry::{ProvidesRegistryState, RegistryState},
    seat::{
        pointer::{PointerEvent, PointerEventKind, PointerHandler},
        relative_pointer::{RelativeMotionEvent, RelativePointerHandler, RelativePointerState},
        Capability, SeatHandler, SeatState,
    },
    shell::wlr_layer::{
        Anchor, KeyboardInteractivity, Layer, LayerShell, LayerShellHandler, LayerSurface,
        LayerSurfaceConfigure,
    },
    shell::WaylandSurface,
    shm::{slot::Buffer as SctkBuffer, slot::SlotPool, Shm, ShmHandler},
};
use std::mem::ManuallyDrop;
use wayland_client::{
    backend::WaylandError,
    protocol::{wl_buffer, wl_output, wl_pointer, wl_region, wl_seat, wl_shm, wl_surface},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::wp::relative_pointer::zv1::client::zwp_relative_pointer_v1::ZwpRelativePointerV1;
use waypenguin_backends::{
    BackendError, DesktopBackend, DesktopWindow, ScreenInfo, WindowGeometry,
};

pub struct CosmicBackend {
    pub connection: Connection,
    pub event_queue: wayland_client::EventQueue<CosmicState>,
    pub qh: QueueHandle<CosmicState>,
    pub state: CosmicState,
}

pub struct CosmicState {
    pub registry_state: RegistryState,
    pub compositor: CompositorState,
    pub shm: Shm,
    pub layer_shell: LayerShell,
    pub output_state: smithay_client_toolkit::output::OutputState,
    pub seat_state: SeatState,
    pub relative_pointer_state: RelativePointerState,
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub cursor_known: bool,
    pub pet_window_x: f64,
    pub pet_window_y: f64,
    pub last_click: Option<(i32, i32)>,
    pub window_geometries: Vec<WindowGeometry>,
}

impl ProvidesRegistryState for CosmicState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    smithay_client_toolkit::registry_handlers![
        smithay_client_toolkit::output::OutputState,
        smithay_client_toolkit::seat::SeatState,
    ];
}

impl CompositorHandler for CosmicState {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _scale_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _transform: wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }
}

impl ShmHandler for CosmicState {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

impl LayerShellHandler for CosmicState {
    fn closed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _layer_surface: &LayerSurface,
    ) {
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _layer_surface: &LayerSurface,
        _configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
    }
}

impl OutputHandler for CosmicState {
    fn output_state(&mut self) -> &mut smithay_client_toolkit::output::OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

impl SeatHandler for CosmicState {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {}

    fn new_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat,
        _capability: Capability,
    ) {
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat,
        _capability: Capability,
    ) {
    }

    fn remove_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {
    }
}

impl PointerHandler for CosmicState {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &wl_pointer::WlPointer,
        events: &[PointerEvent],
    ) {
        for event in events {
            match &event.kind {
                PointerEventKind::Enter { .. } | PointerEventKind::Motion { .. } => {
                    self.cursor_x = self.pet_window_x + event.position.0;
                    self.cursor_y = self.pet_window_y + event.position.1;
                    self.cursor_known = true;
                }
                PointerEventKind::Leave { .. } => {}
                PointerEventKind::Press { .. } => {
                    let cx = self.pet_window_x + event.position.0;
                    let cy = self.pet_window_y + event.position.1;
                    self.last_click = Some((cx as i32, cy as i32));
                }
                _ => {}
            }
        }
    }
}

impl RelativePointerHandler for CosmicState {
    fn relative_pointer_motion(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _relative_pointer: &ZwpRelativePointerV1,
        _pointer: &wl_pointer::WlPointer,
        event: RelativeMotionEvent,
    ) {
        if self.cursor_known {
            self.cursor_x += event.delta.0;
            self.cursor_y += event.delta.1;
        }
    }
}

delegate_compositor!(CosmicState);
delegate_shm!(CosmicState);
delegate_layer!(CosmicState);
delegate_output!(CosmicState);
delegate_seat!(CosmicState);
delegate_pointer!(CosmicState);
delegate_relative_pointer!(CosmicState);
delegate_registry!(CosmicState);

impl Dispatch<wl_region::WlRegion, GlobalData> for CosmicState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_region::WlRegion,
        _event: <wl_region::WlRegion as wayland_client::Proxy>::Event,
        _data: &GlobalData,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl CosmicBackend {
    pub fn new() -> Result<Self, BackendError> {
        let conn = Connection::connect_to_env()
            .map_err(|e| BackendError::InitializationFailed(e.to_string()))?;
        let (globals, mut event_queue) = wayland_client::globals::registry_queue_init(&conn)
            .map_err(|e| BackendError::InitializationFailed(e.to_string()))?;
        let qh = event_queue.handle();

        let compositor_state = CompositorState::bind(&globals, &qh)
            .map_err(|e| BackendError::InitializationFailed(format!("compositor: {:?}", e)))?;
        let shm = Shm::bind(&globals, &qh)
            .map_err(|e| BackendError::InitializationFailed(format!("shm: {:?}", e)))?;
        let layer_shell = LayerShell::bind(&globals, &qh)
            .map_err(|e| BackendError::InitializationFailed(format!("layer_shell: {:?}", e)))?;
        let output_state = smithay_client_toolkit::output::OutputState::new(&globals, &qh);
        let seat_state = SeatState::new(&globals, &qh);
        let relative_pointer_state = RelativePointerState::bind(&globals, &qh);

        let registry_state = RegistryState::new(&globals);

        let mut state = CosmicState {
            registry_state,
            compositor: compositor_state,
            shm,
            layer_shell,
            output_state,
            seat_state,
            relative_pointer_state,
            cursor_x: 0.0,
            cursor_y: 0.0,
            cursor_known: false,
            pet_window_x: 0.0,
            pet_window_y: 0.0,
            last_click: None,
            window_geometries: Vec::new(),
        };

        event_queue
            .roundtrip(&mut state)
            .map_err(|e| BackendError::InitializationFailed(e.to_string()))?;

        let seat = state.seat_state.seats().next();
        if let Some(ref seat) = seat {
            if let Ok(pointer) = state.seat_state.get_pointer(&qh, seat) {
                let _ = state
                    .relative_pointer_state
                    .get_relative_pointer(&pointer, &qh);
            }
        }

        event_queue
            .roundtrip(&mut state)
            .map_err(|e| BackendError::InitializationFailed(e.to_string()))?;

        Ok(Self {
            connection: conn,
            event_queue,
            qh,
            state,
        })
    }

    pub fn pump_events(&mut self) -> Result<(), BackendError> {
        self.event_queue
            .dispatch_pending(&mut self.state)
            .map_err(|e| BackendError::WindowError(format!("dispatch: {e:?}")))?;

        self.event_queue
            .flush()
            .map_err(|e| BackendError::WindowError(format!("flush: {e:?}")))?;

        if let Some(guard) = self.connection.prepare_read() {
            match guard.read() {
                Ok(_) => {
                    self.event_queue
                        .dispatch_pending(&mut self.state)
                        .map_err(|e| BackendError::WindowError(format!("dispatch: {e:?}")))?;
                }
                Err(WaylandError::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(BackendError::WindowError(format!("read: {e:?}"))),
            }
        }
        Ok(())
    }
}

impl DesktopBackend for CosmicBackend {
    fn pump_events(&mut self) -> Result<(), BackendError> {
        CosmicBackend::pump_events(self)
    }

    fn get_screens(&self) -> Vec<ScreenInfo> {
        self.state
            .output_state
            .outputs()
            .filter_map(|output| {
                let info = self.state.output_state.info(&output)?;
                Some(ScreenInfo {
                    name: info.name.clone().unwrap_or_else(|| "Unknown".to_string()),
                    x: info.logical_position.unwrap_or((0, 0)).0,
                    y: info.logical_position.unwrap_or((0, 0)).1,
                    width: info.logical_size.unwrap_or((1920, 1080)).0 as u32,
                    height: info.logical_size.unwrap_or((1920, 1080)).1 as u32,
                    scale_factor: info.scale_factor as f64,
                })
            })
            .collect()
    }

    fn get_cursor_position(&self) -> (i32, i32) {
        (
            if self.state.cursor_known {
                self.state.cursor_x as i32
            } else {
                0
            },
            if self.state.cursor_known {
                self.state.cursor_y as i32
            } else {
                0
            },
        )
    }

    fn get_last_click(&self) -> Option<(i32, i32)> {
        self.state.last_click
    }

    fn clear_last_click(&mut self) {
        self.state.last_click = None;
    }

    fn get_window_geometries(&self) -> Vec<WindowGeometry> {
        self.state.window_geometries.clone()
    }

    fn create_window(
        &mut self,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
    ) -> Result<Box<dyn DesktopWindow>, BackendError> {
        self.state.pet_window_x = x as f64;
        self.state.pet_window_y = y as f64;

        let surface = self.state.compositor.create_surface(&self.qh);

        let layer_surface = self.state.layer_shell.create_layer_surface(
            &self.qh,
            surface,
            Layer::Overlay,
            Some("waypenguin"),
            None,
        );

        layer_surface.set_size(width, height);
        layer_surface.set_anchor(Anchor::TOP | Anchor::LEFT);
        layer_surface.set_margin(y, 0, 0, x);
        layer_surface.set_keyboard_interactivity(KeyboardInteractivity::None);
        layer_surface.wl_surface().commit();

        self.event_queue
            .roundtrip(&mut self.state)
            .map_err(|e| BackendError::WindowCreationFailed(e.to_string()))?;

        let size = (width * height * 4) as usize;
        let aligned = (size + 63) & !63;
        let mut pool = SlotPool::new(aligned * 2, &self.state.shm)
            .map_err(|e| BackendError::WindowCreationFailed(format!("SlotPool: {:?}", e)))?;

        let stride = (width * 4) as i32;
        let format = wl_shm::Format::Argb8888;

        let frames = [
            Self::create_frame(&mut pool, width, height, stride, format)?,
            Self::create_frame(&mut pool, width, height, stride, format)?,
        ];

        Ok(Box::new(CosmicWindow {
            layer_surface,
            _pool: pool,
            width,
            height,
            frames,
            current: 0,
        }))
    }
}

impl CosmicBackend {
    fn create_frame(
        pool: &mut SlotPool,
        width: u32,
        height: u32,
        stride: i32,
        format: wl_shm::Format,
    ) -> Result<FrameBuffer, BackendError> {
        let (buffer, slot) = pool
            .create_buffer(width as i32, height as i32, stride, format)
            .map_err(|e| BackendError::WindowCreationFailed(format!("create_buffer: {:?}", e)))?;

        let canvas_ptr = slot.as_mut_ptr();
        let canvas_len = slot.len();

        Ok(FrameBuffer {
            buffer: ManuallyDrop::new(buffer),
            canvas_ptr,
            canvas_len,
        })
    }
}

pub struct CosmicWindow {
    layer_surface: LayerSurface,
    _pool: SlotPool,
    width: u32,
    height: u32,
    frames: [FrameBuffer; 2],
    current: usize,
}

struct FrameBuffer {
    buffer: ManuallyDrop<SctkBuffer>,
    // Pointer + length into the SlotPool's mmap'd SHM memory.
    // SAFETY: this pointer is valid for the lifetime of the SlotPool
    // (which outlives CosmicWindow since pool stays in CosmicWindow).
    // We only write from the main thread, single-threaded.
    canvas_ptr: *mut u8,
    canvas_len: usize,
}

impl FrameBuffer {
    fn write_pixels(&mut self, pixels: &[u32]) {
        let dst = unsafe {
            std::slice::from_raw_parts_mut(self.canvas_ptr as *mut u32, self.canvas_len / 4)
        };
        let copy_len = pixels.len().min(dst.len());
        dst[..copy_len].copy_from_slice(&pixels[..copy_len]);
    }

    fn wl_buffer(&self) -> &wl_buffer::WlBuffer {
        self.buffer.wl_buffer()
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.buffer);
        }
    }
}

impl DesktopWindow for CosmicWindow {
    fn set_size(&mut self, width: u32, height: u32) -> Result<(), BackendError> {
        self.width = width;
        self.height = height;
        self.layer_surface.set_size(width, height);
        self.layer_surface.wl_surface().commit();
        Ok(())
    }

    fn set_position(&mut self, x: i32, y: i32) -> Result<(), BackendError> {
        self.layer_surface.set_margin(y, 0, 0, x);
        Ok(())
    }

    fn present_pixels(&mut self, pixels: &[u32]) -> Result<(), BackendError> {
        let width = self.width;
        let height = self.height;

        self.current = (self.current + 1) & 1;
        let fb = &mut self.frames[self.current];

        fb.write_pixels(pixels);

        let surface = self.layer_surface.wl_surface();
        if fb.buffer.attach_to(surface).is_err() {
            surface.attach(Some(fb.wl_buffer()), 0, 0);
        }
        surface.damage_buffer(0, 0, width as i32, height as i32);
        surface.commit();

        Ok(())
    }

    fn set_click_through(&mut self, _click_through: bool) -> Result<(), BackendError> {
        Ok(())
    }
}
