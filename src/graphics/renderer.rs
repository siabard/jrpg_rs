use sdl2::{
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

pub struct Renderer<'a> {
    canvas: &'a mut WindowCanvas,
}

impl<'a> Renderer<'a> {
    pub fn new(canvas: &'a mut WindowCanvas) -> Renderer<'a> {
        Renderer { canvas }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
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
