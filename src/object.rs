use piston_window::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;

pub struct Object {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Object {
    pub fn new() -> Object {
        Object {x : 0.0, y : 0.0}
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    // Now, we also make our object do the rendering

    pub fn render<G>(&self, g: &mut G, view: math::Matrix2d) where G: Graphics {
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        rectangle(red, square, view.trans(self.x, self.y).trans(-50.0, -50.0), g);
    }
}