use sdl2::rect::Rect;

pub fn follow(camera: &Rect, target: &Rect, ratio: f64) -> Rect {
    // 상하좌우에 대해서 해당 ratio안에
    // 값을 넣도록 하는 방식
    let inner_rect = Rect::new(
        (camera.x as f64 + (camera.width() as f64 * ratio)) as i32,
        (camera.y as f64 + (camera.height() as f64 * ratio)) as i32,
        (camera.width() as f64 * (1.0 - ratio * 2.0)) as u32,
        (camera.height() as f64 * (1.0 - ratio * 2.0)) as u32,
    );

    // inner_rect를 해당 (target.x, target.y)이 벗어난다면
    // 그때에는 그 차이만큼 inner_rect를 보정해주어야한다.

    let target_point = target.center();

    if inner_rect.contains_point(target_point) {
        return *camera;
    }

    // center에서 얼마나 차이나는지 확인
    let camera_center = camera.center();
    let mut camera_point = *camera;

    if camera_center.x > target_point.x && camera.left() > target.left() {
        // camera의 x위치를 target 에서 inner_rect의 left 차이만큼 이동한다.
        camera_point.x = 0.max(camera.left() - (camera_point.left() - target.left()));
    }

    if camera_center.x < target_point.x && camera.right() < target.right() {
        camera_point.x = 0.max(camera.left() + (target.right() - camera_point.right()));
    }

    if camera_center.y > target_point.y && camera.top() > target.top() {
        camera_point.y = 0.max(camera.top() - (camera_point.top() - target.top()));
    }

    if camera_center.y < target_point.y && camera.bottom() < target.bottom() {
        camera_point.y = 0.max(camera.top() + (target.bottom() - camera_point.bottom()));
    }

    camera_point
}
