use std::path::Path;

use jrpg_rs::graphics::*;
use jrpg_rs::map::*;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::rect::Rect;

fn main() -> Result<(), &'static str> {
    let sdl_context = sdl2::init().expect("ERR::MAIN::SDL2_INIT");
    let video_subsystem = sdl_context.video().expect("ERR::MAIN::INIT_VIDEO");
    let _image_subsystem = sdl2::image::init(InitFlag::all());
    let timer_subsystem = sdl_context.timer().expect("ERR::MAIN::INIT_TIMER");

    let window = video_subsystem
        .window("JPRG", 800, 600)
        .position_centered()
        .build()
        .expect("ERR::MAIN::INIT_WINDOW");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("ERR::MAIN::INIT::CANVAS");

    let texture_creator = canvas.texture_creator();

    let mut camera_rect = Rect::new(0, 0, 320, 200);
    let mut camera_buffer = texture_creator
        .create_texture_target(
            PixelFormatEnum::RGBA8888,
            camera_rect.width(),
            camera_rect.height(),
        )
        .unwrap();

    let mut textures = TextureManager::new();
    textures.load_texture(&texture_creator, "player", &Path::new("assets/mychar.png"));

    let mut renderer = Renderer::new(&mut canvas);

    let mut player = Sprite::new(
        Rect::new(0, 0, 32, 32),
        textures.get_texture("player").as_ref().unwrap(),
        100,
        100,
    );
    let map = Map::new(
        "world_map".to_owned(),
        &texture_creator,
        "tiled_base64_zlib.tmx",
    );
    let mut event_pump = sdl_context
        .event_pump()
        .expect("ERR::MAIN::INIT_EVENT_PUMP");

    let mut dt: f64;
    let mut last_time: u32 = 0;
    let mut now: u32 = timer_subsystem.ticks();

    'running: loop {
        dt = (now - last_time) as f64 / 1000.0;
        last_time = now;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(k), ..
                } => match k {
                    Keycode::Up => {
                        camera_rect.y -= 1;
                    }
                    Keycode::Down => {
                        camera_rect.y += 1;
                    }
                    Keycode::Left => {
                        camera_rect.x -= 1;
                    }
                    Keycode::Right => {
                        camera_rect.x += 1;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        player.x += (300.0 * dt) as i32;
        player.y += (300.0 * dt) as i32;

        renderer.clear();

        map.render(&mut renderer, &camera_rect, &mut camera_buffer);
        player.render(&mut renderer, &camera_rect);

        //renderer.draw_rect(camera_rect);

        renderer.present();
        now = timer_subsystem.ticks();
    }

    Ok(())
}
