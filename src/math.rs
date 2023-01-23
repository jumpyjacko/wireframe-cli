use crate::*;

// All three of these rotation functions use the simplified per-axis rotation matrix
pub fn simple_rotate_x(point: &Point3D, theta: &f32) -> Point3D {
    let x = point.x;
    let yz = arr1(&[point.y, point.z]);
    let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

    let rotated_yz = matrix.dot(&yz);

    Point3D {
        x,
        y: rotated_yz[0],
        z: rotated_yz[1],
    }
}

pub fn simple_rotate_y(point: &Point3D, theta: &f32) -> Point3D {
    let y = point.y;
    let xz = arr1(&[point.x, point.z]);
    let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

    let rotated_xz = matrix.dot(&xz);

    Point3D {
        x: rotated_xz[0],
        y,
        z: rotated_xz[1],
    }
}

pub fn simple_rotate_z(point: &Point3D, theta: &f32) -> Point3D {
    let z = point.z;
    let xy = arr1(&[point.x, point.y]);
    let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

    let rotated_xy = matrix.dot(&xy);

    Point3D {
        x: rotated_xy[0],
        y: rotated_xy[1],
        z,
    }
}
