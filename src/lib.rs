pub extern crate event_loop;
pub extern crate graphics;
pub extern crate input;
pub extern crate window;

use std::error::Error;
use std::time::Duration;

use event_loop::{EventLoop, EventSettings, Events};
use graphics::types::Color;
use graphics::{DrawState, Graphics, Viewport};
use input::Event;
use opengl_graphics::{GlGraphics, OpenGL};
use sdl2_window::Sdl2Window;
use window::{AdvancedWindow, Api, Position, Size, Window, WindowSettings};

const OPENGL_VERSION: OpenGL = OpenGL::V2_0;

pub struct Context {
    events: Events,
    window: Sdl2Window,
    graphics: GlGraphics,
}

impl Context {
    pub fn new(
        event_settings: EventSettings,
        window_settings: WindowSettings,
    ) -> Result<Context, Box<dyn Error>> {
        let (major, minor) = OPENGL_VERSION.get_major_minor();
        Ok(Context {
            events: Events::new(event_settings),
            window: window_settings
                .graphics_api(Api::opengl(major as u32, minor as u32))
                .build()?,
            graphics: GlGraphics::new(OPENGL_VERSION),
        })
    }

    pub fn next(&mut self) -> Option<Event> {
        self.events.next(&mut self.window)
    }

    pub fn draw_begin(&mut self, viewport: Viewport) -> graphics::Context {
        self.graphics.draw_begin(viewport)
    }

    pub fn draw_end(&mut self) {
        self.graphics.draw_end();
    }

    pub fn draw<F, R>(&mut self, viewport: Viewport, f: F) -> R
    where
        F: FnOnce(graphics::Context, &mut Self) -> R,
    {
        let context = self.draw_begin(viewport);
        let retval = f(context, self);
        self.draw_end();
        retval
    }
}

impl EventLoop for Context {
    fn get_event_settings(&self) -> EventSettings {
        self.events.get_event_settings()
    }

    fn set_event_settings(&mut self, settings: EventSettings) {
        self.events.set_event_settings(settings)
    }
}

impl Window for Context {
    fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }

    fn should_close(&self) -> bool {
        self.window.should_close()
    }

    fn size(&self) -> Size {
        self.window.size()
    }

    fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    fn wait_event(&mut self) -> Event {
        self.window.wait_event()
    }

    fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Event> {
        self.window.wait_event_timeout(timeout)
    }

    fn poll_event(&mut self) -> Option<Event> {
        self.window.poll_event()
    }

    fn draw_size(&self) -> Size {
        self.window.draw_size()
    }
}

impl AdvancedWindow for Context {
    fn get_title(&self) -> String {
        self.window.get_title()
    }

    fn set_title(&mut self, value: String) {
        self.window.set_title(value);
    }

    fn get_exit_on_esc(&self) -> bool {
        self.window.get_exit_on_esc()
    }

    fn set_exit_on_esc(&mut self, value: bool) {
        self.window.set_exit_on_esc(value);
    }

    fn get_automatic_close(&self) -> bool {
        self.window.get_automatic_close()
    }

    fn set_automatic_close(&mut self, value: bool) {
        self.window.set_automatic_close(value);
    }

    fn set_capture_cursor(&mut self, value: bool) {
        self.window.set_capture_cursor(value);
    }

    fn show(&mut self) {
        self.window.show();
    }

    fn hide(&mut self) {
        self.window.hide();
    }

    fn get_position(&self) -> Option<Position> {
        self.window.get_position()
    }

    fn set_position<P: Into<Position>>(&mut self, val: P) {
        self.window.set_position(val);
    }

    fn set_size<S: Into<Size>>(&mut self, val: S) {
        self.window.set_size(val);
    }
}

impl Graphics for Context {
    type Texture = <GlGraphics as Graphics>::Texture;

    fn clear_color(&mut self, color: Color) {
        self.graphics.clear_color(color);
    }

    fn clear_stencil(&mut self, value: u8) {
        self.graphics.clear_stencil(value);
    }

    fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], f: F)
    where
        F: FnMut(&mut dyn FnMut(&[[f32; 2]])),
    {
        self.graphics.tri_list(draw_state, color, f);
    }

    fn tri_list_uv<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &Self::Texture,
        f: F,
    ) where
        F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]])),
    {
        self.graphics.tri_list_uv(draw_state, color, texture, f);
    }
}
