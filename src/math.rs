use crate::*;

#[derive(Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point3D {
    pub fn project(&self, focal_length: &i32) -> Point2D {
        let p_x: i32 =
            ((*focal_length as f32 * self.x) / (*focal_length as f32 + self.z + 256.0)) as i32;
        let p_y: i32 =
            ((*focal_length as f32 * self.y) / (*focal_length as f32 + self.z + 256.0)) as i32;

        let projected_point: Point2D = Point2D { x: p_x, y: p_y };

        projected_point
    }

    // All three of these rotation functions use the simplified per-axis rotation matrix
    pub fn rotate_x(&self, theta: &f32) -> Point3D {
        let yz = arr1(&[self.y, self.z]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

        let rotated_yz = matrix.dot(&yz);

        Point3D {
            x: self.x,
            y: rotated_yz[0],
            z: rotated_yz[1],
        }
    }

    pub fn rotate_y(&self, theta: &f32) -> Point3D {
        let xz = arr1(&[self.x, self.z]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

        let rotated_xz = matrix.dot(&xz);

        Point3D {
            x: rotated_xz[0],
            y: self.y,
            z: rotated_xz[1],
        }
    }

    pub fn rotate_z(&self, theta: &f32) -> Point3D {
        let xy = arr1(&[self.x, self.y]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

        let rotated_xy = matrix.dot(&xy);

        Point3D {
            x: rotated_xy[0],
            y: rotated_xy[1],
            z: self.z,
        }
    }
}

// FIXME: Everything renders upside down, probably due to the way I index and then plot
pub fn plot_line(
    point1: Point2D,
    point2: Point2D,
    mut state: [[u8; SIZE]; SIZE],
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
        state[(y + offset) as usize][(x + offset) as usize] = 1;
        x += dx;
        y += dy;
        i += 1;
    }

    state
}
