use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

pub struct Renderer<'a> {
    pub canvas: &'a mut WindowCanvas,
}

impl<'a> Renderer<'a> {
    pub fn new(canvas: &'a mut WindowCanvas) -> Renderer<'a> {
        Renderer { canvas }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();
    }

    pub fn draw_rect(&mut self, rect: Rect) {
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 200));
        self.canvas.draw_rect(rect).unwrap();
    }

    pub fn render(
        &mut self,
        texture: &Texture,
        src: Rect,
        dst: Rect,
        angle: f64,
        center: Point,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) {
        self.canvas
            .copy_ex(
                texture,
                src,
                dst,
                angle,
                center,
                flip_horizontal,
                flip_vertical,
            )
            .unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
