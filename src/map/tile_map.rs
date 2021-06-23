use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::rect::Rect;

use std::collections::HashMap;

use crate::constants::*;
use crate::tile;
use crate::Renderer;
/// Tiled를 읽어서 맵을 채운다.
use sdl2::video::WindowContext;
use sdl2::{image::LoadTexture, render::Texture, render::TextureCreator};
use std::path::Path;
use tiled::parse_file;

/// 맵의 가로 타일 수
pub const MAP_WIDTH: i32 = 20;

/// 맵의 세로 타일 수
pub const MAP_HEIGHT: i32 = 15;

/// 지도용 구조체
/// 지도에는 map용 파일과
/// 각 map 블럭에 대한 정보를 넣는다.
pub struct Map<'a> {
    pub map_id: String,
    pub tile_atlases: HashMap<usize, tile::TileAtlas>,
    pub width: u32,  // total number of tile in horizontal in this map
    pub height: u32, // total numbr of tile in vertical in this map
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_widths: HashMap<usize, u32>, // width of a tile in pixels
    pub tile_heights: HashMap<usize, u32>, // height of a tile in pixels
    pub layers: Vec<tiled::Layer>,
    pub textures: HashMap<usize, Texture<'a>>,
    pub blocks: Vec<Rect>,
    pub gids: HashMap<u32, usize>,
}

impl<'a> Map<'a> {
    pub fn new(
        map_id: String,
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &'static str,
    ) -> Map<'a> {
        // read tmx file
        let map: tiled::Map = parse_file(&Path::new(&(ASSET_DIR.to_owned() + path))).unwrap();

        let layers: Vec<tiled::Layer> = map.layers;
        let tile_sets: Vec<tiled::Tileset> = map.tilesets;

        let mut textures = HashMap::new();
        let mut tile_atlases = HashMap::new();
        let mut tile_widths = HashMap::new();
        let mut tile_heights = HashMap::new();

        let mut gids = HashMap::new();
        gids.insert(0, 0);

        for (i, tileset) in tile_sets.iter().enumerate() {
            let tile_width = tileset.tile_width;
            let tile_height = tileset.tile_height;

            let texture = texture_creator
                .load_texture(Path::new(
                    &(ASSET_DIR.to_owned() + &tileset.images[0].source),
                ))
                .unwrap();

            let tile_atlas =
                tile::TileAtlas::new(&texture, tileset.first_gid, tile_width, tile_height);
            textures.insert(i, texture);

            //tile atlas에 정의된 모든 tile 정보에 texture id를 넣는다.
            for (j, _) in tile_atlas.atlas.iter().enumerate() {
                gids.insert(j as u32 + tileset.first_gid, i);
            }

            tile_atlases.insert(i, tile_atlas);
            tile_widths.insert(i, tileset.tile_width);
            tile_heights.insert(i, tileset.tile_height);
        }

        // layer의 이름이 collision인 경우에는 해당하는 값의 좌표를 blocks에 넣는다.
        let mut blocks = vec![];

        for (_, layer) in layers.iter().enumerate() {
            if let tiled::LayerData::Finite(tiles) = &layer.tiles {
                if layer.name == "collision" {
                    for y in 0..map.height {
                        for x in 0..map.width {
                            let gid = tiles[y as usize][x as usize].gid;
                            if gid != 0 {
                                blocks.push(Rect::new(
                                    (x * map.tile_width) as i32,
                                    (y * map.tile_height) as i32,
                                    map.tile_width,
                                    map.tile_height,
                                ));
                            }
                        }
                    }
                }
            }
        }

        Map {
            map_id,
            tile_atlases,
            width: map.width,
            height: map.height,
            tile_width: map.tile_width,
            tile_height: map.tile_height,
            tile_widths,
            tile_heights,
            layers,
            textures,
            blocks,
            gids,
        }
    }

    /// translate position (left, top) to tile
    /// map is positioned on (0, 0)
    pub fn point_to_tile(&self, tile_index: usize, left: i32, top: i32, ceilp: bool) -> (i32, i32) {
        let o_x = 0.max(left);
        let o_y = 0.max(top);

        let tile_width = *self.tile_widths.get(&tile_index).unwrap();
        let tile_height = *self.tile_heights.get(&tile_index).unwrap();
        let clamp_x =
            o_x.min(left + (self.width * tile_width) as i32 - (if ceilp { 0 } else { 1 }));
        let clamp_y =
            o_y.min(top + (self.height * tile_height) as i32 - (if ceilp { 0 } else { 1 }));

        let tile_x = if ceilp {
            (clamp_x as f64 / tile_width as f64).ceil() as i32
        } else {
            clamp_x / tile_width as i32
        };

        let tile_y = if ceilp {
            (clamp_y as f64 / tile_height as f64).ceil() as i32
        } else {
            clamp_y / tile_height as i32
        };

        (tile_x, tile_y)
    }

    /// translate (tile_x, tile_y)
    /// to coordinates
    pub fn get_tile_xy(&self, tile_x: u32, tile_y: u32) -> (f64, f64) {
        let tile_x: u32 = tile_x.min(self.width);
        let tile_y: u32 = tile_y.min(self.height);

        (
            (tile_x * self.tile_width) as f64,
            (tile_y * self.tile_height) as f64,
        )
    }

    pub fn render(
        &self,
        renderer: &mut Renderer,
        camera_rect: &Rect,
        camera_buffer: &mut Texture<'a>,
    ) {
        renderer
            .canvas
            .with_texture_canvas(camera_buffer, |texture_canvas| {
                texture_canvas.clear();
                for (i, layer) in self.layers.iter().enumerate() {
                    if layer.name != "collision" {
                        if let tiled::LayerData::Finite(tiles) = &layer.tiles {
                            let (tile_left, tile_top) =
                                self.point_to_tile(i, camera_rect.x, camera_rect.y, false);
                            let (tile_right, tile_bottom) = self.point_to_tile(
                                i,
                                camera_rect.x + camera_rect.w,
                                camera_rect.y + camera_rect.h,
                                true,
                            );

                            // 카메라의 좌측/위가 타일에 정확히 일치한다면 tile_start_x와 tile_start_y는 0이 되겠지만
                            // 그렇지 않은 경우는 좌측/위 타일에서 떨어진 좌표값만큼을 반환하게된다.
                            // 이 값은 대상 texture의 영역을 어디에 노출시킬까 정할 때, 대상의 타일을 tile_start_x, tile_start_y만큼
                            // 좌상단으로 올림으로써 부드러운 스크롤을 가능하게한다.
                            let tile_width = *self.tile_widths.get(&i).unwrap();
                            let tile_height = *self.tile_heights.get(&i).unwrap();

                            let tile_start_x = camera_rect.x - tile_left * tile_width as i32;
                            let tile_start_y = camera_rect.y - tile_top * tile_height as i32;

                            for y in tile_top..tile_bottom {
                                for x in tile_left..tile_right {
                                    let gid = tiles[y as usize][x as usize].gid;
                                    if gid != 0 {
                                        // gid 로 부터 tile_atlases의 index를 구함
                                        // tile_atlases의 모든 first_gid 중 gid 값보다 큰 것 중에 가장 작은 인덱스를 구할 것
                                        // 해당 인덱스가 tile_atlases의 인덱스이다.
                                        // TODO : 이와 같은 방식은 비 경제적이다.
                                        // tile_atlas를 생성할 때, 어떤 texutre index인지, 그리고 해당 texture의 어떤 위치인지를
                                        // 등록하는 편이 좋다.
                                        // 즉 말하자면 Vector이면 되지, 굳이 HashMap일 필요가 없다.
                                        // Vec<(texture_idx: usize, x, y, w, h)> 이면 됨..
                                        let idx_gid = self.gids.get(&gid).unwrap();

                                        let rect = self
                                            .tile_atlases
                                            .get(&idx_gid)
                                            .unwrap()
                                            .get_tile_rect(gid);

                                        texture_canvas
                                            .copy_ex(
                                                &self.textures[&idx_gid],
                                                rect,
                                                Rect::new(
                                                    (x - tile_left) as i32 * tile_width as i32
                                                        - tile_start_x,
                                                    (y - tile_top) as i32 * tile_height as i32
                                                        - tile_start_y,
                                                    tile_width,
                                                    tile_height,
                                                ),
                                                0.0,
                                                Point::new(0, 0),
                                                false,
                                                false,
                                            )
                                            .unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            })
            .unwrap();

        renderer.render(
            camera_buffer,
            Rect::new(0, 0, camera_rect.width(), camera_rect.height()),
            Rect::new(0, 0, camera_rect.width(), camera_rect.height()),
            0.0,
            Point::new(0, 0),
            false,
            false,
        );
    }
}
