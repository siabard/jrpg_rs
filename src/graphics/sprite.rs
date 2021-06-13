use sdl2::rect::Rect;

pub struct Sprite {
    pub rect: Rect,
    pub x: i32,
    pub y: i32,
}

impl Sprite {
    pub fn new(rect: Rect, x: i32, y: i32) -> Sprite {
        Sprite { rect, x, y }
    }
}
