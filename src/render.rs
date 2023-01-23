use crate::*;

pub fn project_point(point: &Point3D, focal_length: &i32) -> Point2D {
    let p_x: i32 =
        ((*focal_length as f32 * point.x) / (*focal_length as f32 + point.z + 256.0)) as i32;
    let p_y: i32 =
        ((*focal_length as f32 * point.y) / (*focal_length as f32 + point.z + 256.0)) as i32;

    let projected_point: Point2D = Point2D { x: p_x, y: p_y };

    projected_point
}

// FIXME: Everything renders upside down, probably due to the way I index and then plot
pub fn plot_line(
    point1: Point2D,
    point2: Point2D,
    mut screen: [[u8; SIZE]; SIZE],
) -> [[u8; SIZE]; SIZE] {
    let offset: f32 = SIZE as f32 / 2.0;

    // Digital Differential Analyzer Line Drawing Algorithm
    let mut dx: f32 = point2.x as f32 - point1.x as f32;
    let mut dy: f32 = point2.y as f32 - point1.y as f32;
    let step = if dx.abs() >= dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };
    dx /= step;
    dy /= step;
    let mut x = point1.x as f32;
    let mut y = point1.y as f32;
    let mut i = 1;
    while i as f32 <= step {
        screen[(y + offset) as usize][(x + offset) as usize] = 1;
        x += dx;
        y += dy;
        i += 1;
    }

    screen
}
