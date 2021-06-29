use std::path::Path;
use std::time::Duration;

use jrpg_rs::camera::follow;
use jrpg_rs::graphics::*;
use jrpg_rs::input::Input;
use jrpg_rs::map::*;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Scancode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

fn main() -> Result<(), &'static str> {
    let sdl_context = sdl2::init().expect("ERR::MAIN::SDL2_INIT");
    let video_subsystem = sdl_context.video().expect("ERR::MAIN::INIT_VIDEO");
    let _image_subsystem = sdl2::image::init(InitFlag::all());
    let timer_subsystem = sdl_context.timer().expect("ERR::MAIN::INIT_TIMER");

    let mut input = Input::default();
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
        0.5,
    );

    player.add_animation("LEFT".to_string(), Rect::new(0, 0, 16, 16), vec![0, 1, 2]);
    player.set_key("LEFT".to_string());

    player.add_animation("RIGHT".to_string(), Rect::new(0, 16, 16, 16), vec![0, 1, 2]);
    player.set_key("RIGHT".to_string());

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

        input.begin_new_frame();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { repeat, .. } if !repeat => {
                    input.key_down_event(&event);
                }
                Event::KeyUp { .. } => input.key_up_event(&event),
                _ => {}
            }
        }

        if input.is_key_held(Scancode::Up) {
            player.y -= (300.0 * dt) as i32;
        }
        if input.is_key_held(Scancode::Down) {
            player.y += (300.0 * dt) as i32;
        }
        if input.is_key_held(Scancode::Left) {
            player.x -= (300.0 * dt) as i32;
            player.set_key("LEFT".to_string());
        }
        if input.is_key_held(Scancode::Right) {
            player.x += (300.0 * dt) as i32;
            player.set_key("RIGHT".to_string());
        }
        if input.was_key_pressed(Scancode::Q) {
            break 'running;
        }

        let player_rect = Rect::new(
            player.x,
            player.y,
            player.rect.width(),
            player.rect.height(),
        );

        camera_rect = follow(&camera_rect, &player_rect, 0.2);

        renderer.clear();

        map.render(&mut renderer, &camera_rect, &mut camera_buffer);
        player.update(dt);
        player.render(&mut renderer, &camera_rect);

        renderer.draw_rect(Rect::new(0, 0, camera_rect.width(), camera_rect.height()));

        renderer.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        now = timer_subsystem.ticks();
    }

    Ok(())
}
