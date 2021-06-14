use std::path::Path;

use jrpg_rs::graphics::*;
use jrpg_rs::map::*;

use sdl2::event::Event;
use sdl2::image::InitFlag;
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

    let mut textures = TextureManager::new();
    textures.load_texture(&texture_creator, "player", &Path::new("assets/mychar.png"));

    let mut renderer = Renderer::new(&mut canvas);

    let mut player = Sprite::new(Rect::new(0, 0, 32, 32), 100, 100);
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
                _ => {}
            }
        }

        player.x += (300.0 * dt) as i32;
        player.y += (300.0 * dt) as i32;

        renderer.clear();

        renderer.render(
            textures.get_texture("player").unwrap(),
            player.rect,
            Rect::new(player.x, player.y, 32, 32),
            0.0,
            Point::new(0, 0),
            false,
            false,
        );

        map.render(&mut renderer, &Rect::new(0, 0, 320, 200));
        renderer.present();
        now = timer_subsystem.ticks();
    }

    Ok(())
}
