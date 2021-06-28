use sdl2::{
    rect::{Point, Rect},
    render::Texture,
};

use crate::Renderer;
use std::collections::HashMap;

pub struct Sprite<'a> {
    pub rect: Rect,
    pub frames: HashMap<String, Vec<Rect>>,
    pub x: i32,
    pub y: i32,
    key: String,
    current_frame: i32,
    delta: f64,
    duration: f64,
    texture: &'a Texture<'a>,
}

impl<'a> Sprite<'a> {
    pub fn new(rect: Rect, texture: &'a Texture<'a>, x: i32, y: i32, duration: f64) -> Sprite {
        Sprite {
            rect,
            frames: HashMap::new(),
            x,
            y,
            key: String::from(""),
            current_frame: 0,
            delta: 0.0,
            duration,
            texture,
        }
    }

    pub fn add_animation(&mut self, key: String, rect: Rect, frames: Vec<i32>) {
        let mut rects: Vec<Rect> = vec![];
        for frame in frames {
            rects.push(Rect::new(
                rect.x + rect.width() as i32 * frame,
                rect.y,
                rect.width(),
                rect.height(),
            ));
        }
        self.frames.insert(key, rects);
    }

    pub fn set_key(&mut self, key: String) {
        if self.frames.contains_key(&key) {
            self.key = key;
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.delta += dt;
        if self.delta >= self.duration {
            self.delta = 0.0;
            self.current_frame += 1;
            if self.current_frame >= self.frames.get(&self.key).unwrap().len() as i32 {
                self.current_frame = 0;
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer, camera_rect: &Rect) {
        let rect = self.frames.get(&self.key).unwrap()[self.current_frame as usize];

        renderer.render(
            self.texture,
            rect,
            Rect::new(self.x - camera_rect.x, self.y - camera_rect.y, 32, 32),
            0.0,
            Point::new(0, 0),
            false,
            false,
        );
    }
}
