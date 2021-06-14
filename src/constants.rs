/// 실제 물리화면의 가로크기
pub const SCREEN_WIDTH: u32 = 800;

/// 실제 물리화면의 세로크기
pub const SCREEN_HEIGHT: u32 = 600;

/// 카메라로 보이는 VIEWPORT의 크기는 VIRTUAL_* 인 것으로 가정한다.
/// 게임이 진행되는 화면의 논리적 가로크기
pub const VIRTUAL_WIDTH: u32 = 800;

/// 게임이 진행되는 화면의 논리적 세로크기
pub const VIRTUAL_HEIGHT: u32 = 600;

/// 게임의 모든 텍스쳐는 아래 좌표계에 맞추어 보이게 된다.
/// 게임이 진행되는 전체 영역의 가로 크기
pub const WORLD_WIDTH: u32 = 1600;

/// 게임이 진행되는 전체 영역의 새로 크기
pub const WORLD_HEIGHT: u32 = 1200;

/// 물리 화면과 VIEWPORT 화면과의 확대 축소를 위한 가로 비율
pub const WIDTH_RATIO: f32 = SCREEN_WIDTH as f32 / VIRTUAL_WIDTH as f32;

/// 물리 화면과 VIEWPORT 화면과의 확대 축소를 위한 세로 비율
pub const HEIGHT_RATIO: f32 = SCREEN_HEIGHT as f32 / VIRTUAL_HEIGHT as f32;

/// 물리 화면과 VIEWPORT 화면과의 확대 축소를 위한 가로 비율의 역값
pub const REVERSE_WIDTH_RATIO: f32 = 1.0 / WIDTH_RATIO;

/// 물리 화면과 VIEWPORT 화면과의 확대 축소를 위한 세로 비율의 역값
pub const REVERSE_HEIGHT_RATIO: f32 = 1.0 / HEIGHT_RATIO;

/// 1/60 초에 대한 마이크로초 (안쓰임)
pub const TIME_SPAN: u32 = 1_000_000_000 / 60;

/// delta t설정하기 (안쓰임)
pub const DELTA_T: f64 = 1.0f64 / 60.0;

/// asset 파일이 들어있는 곳
pub const ASSET_DIR: &'static str = "assets/";
