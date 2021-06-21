use sdl2::{
    rect::{Point, Rect},
    render::Texture,
};

use crate::Renderer;

pub struct Sprite<'a> {
    pub rect: Rect,
    pub x: i32,
    pub y: i32,
    texture: &'a Texture<'a>,
}

impl<'a> Sprite<'a> {
    pub fn new(rect: Rect, texture: &'a Texture<'a>, x: i32, y: i32) -> Sprite {
        Sprite {
            rect,
            x,
            y,
            texture,
        }
    }

    pub fn render(&self, renderer: &mut Renderer, camera_rect: &Rect) {
        renderer.render(
            self.texture,
            self.rect,
            Rect::new(self.x - camera_rect.x, self.y - camera_rect.y, 32, 32),
            0.0,
            Point::new(0, 0),
            false,
            false,
        );
    }
}
