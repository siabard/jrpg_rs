use sdl2::{rect::Rect, render::Texture};

use crate::{Renderer, Sprite};

pub struct Player<'a> {
    sprite: Sprite<'a>,
}

impl<'a> Player<'a> {
    pub fn new(texture: &'a Texture<'a>) -> Player<'a> {
        let mut sprite = Sprite::new(Rect::new(0, 0, 32, 32), texture, 0, 0, 0.5);

        sprite.add_animation("LEFT".to_string(), Rect::new(0, 0, 16, 16), vec![0, 1, 2]);
        sprite.set_key("LEFT".to_string());

        sprite.add_animation("RIGHT".to_string(), Rect::new(0, 16, 16, 16), vec![0, 1, 2]);
        sprite.set_key("RIGHT".to_string());

        Player { sprite }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.sprite.x,
            self.sprite.y,
            self.sprite.rect.width(),
            self.sprite.rect.height(),
        )
    }

    pub fn move_x(&mut self, amount: i32) {
        self.sprite.x += amount;
        if amount < 0 {
            self.sprite.set_key("LEFT".to_string());
        } else if amount > 0 {
            self.sprite.set_key("RIGHT".to_string());
        }
    }

    pub fn move_y(&mut self, amount: i32) {
        self.sprite.y += amount;
    }

    pub fn update(&mut self, dt: f64) {
        self.sprite.update(dt);
    }

    pub fn render(&self, renderer: &mut Renderer, rect: &Rect) {
        self.sprite.render(renderer, rect);
    }
}
