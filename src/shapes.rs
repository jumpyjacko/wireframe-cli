use crate::*;

#[derive(Clone, Copy)]
pub struct Shape<'a> {
    pub verts: &'a [Point3D],
    pub edges: &'a [Edge],
}

pub const CUBE: Shape = Shape {
    verts: &[
        Point3D {
            x: -60.0,
            y: 60.0,
            z: 60.0,
        },
        Point3D {
            x: 60.0,
            y: 60.0,
            z: 60.0,
        },
        Point3D {
            x: 60.0,
            y: -60.0,
            z: 60.0,
        },
        Point3D {
            x: -60.0,
            y: -60.0,
            z: 60.0,
        },
        Point3D {
            x: -60.0,
            y: 60.0,
            z: -60.0,
        },
        Point3D {
            x: 60.0,
            y: 60.0,
            z: -60.0,
        },
        Point3D {
            x: 60.0,
            y: -60.0,
            z: -60.0,
        },
        Point3D {
            x: -60.0,
            y: -60.0,
            z: -60.0,
        },
    ],
    edges: &[
        // Front Face
        Edge { a: 0, b: 1 },
        Edge { a: 1, b: 2 },
        Edge { a: 2, b: 3 },
        Edge { a: 3, b: 0 },
        // Back Face
        Edge { a: 4, b: 5 },
        Edge { a: 5, b: 6 },
        Edge { a: 6, b: 7 },
        Edge { a: 7, b: 4 },
        // Connecting the Front and Back Face
        Edge { a: 0, b: 4 },
        Edge { a: 1, b: 5 },
        Edge { a: 2, b: 6 },
        Edge { a: 3, b: 7 },
    ],
};

pub const PYRAMID: Shape = Shape {
    verts: &[
        Point3D {
            x: -60.0,
            y: -60.0,
            z: -60.0,
        },
        Point3D {
            x: 60.0,
            y: -60.0,
            z: -60.0,
        },
        Point3D {
            x: 60.0,
            y: -60.0,
            z: 60.0,
        },
        Point3D {
            x: -60.0,
            y: -60.0,
            z: 60.0,
        },
        Point3D {
            x: 0.0,
            y: 60.0,
            z: 0.0,
        },
    ],
    edges: &[
        // Base Face
        Edge { a: 0, b: 1 },
        Edge { a: 1, b: 2 },
        Edge { a: 2, b: 3 },
        Edge { a: 3, b: 0 },
        // Base-to-Point Edges
        Edge { a: 0, b: 4 },
        Edge { a: 1, b: 4 },
        Edge { a: 2, b: 4 },
        Edge { a: 3, b: 4 },
    ],
};

pub const STAR_CUBE: Shape = Shape {
    verts: &[
        Point3D {
            x: -70.0,
            y: 70.0,
            z: 70.0,
        },
        Point3D {
            x: 70.0,
            y: 70.0,
            z: 70.0,
        },
        Point3D {
            x: 70.0,
            y: -70.0,
            z: 70.0,
        },
        Point3D {
            x: -70.0,
            y: -70.0,
            z: 70.0,
        },
        Point3D {
            x: -70.0,
            y: 70.0,
            z: -70.0,
        },
        Point3D {
            x: 70.0,
            y: 70.0,
            z: -70.0,
        },
        Point3D {
            x: 70.0,
            y: -70.0,
            z: -70.0,
        },
        Point3D {
            x: -70.0,
            y: -70.0,
            z: -70.0,
        },
    ],
    edges: &[
        // Connecting Diagonals across the faces
        Edge { a: 0, b: 2 },
        Edge { a: 0, b: 5 },
        Edge { a: 1, b: 3 },
        Edge { a: 1, b: 4 },
        Edge { a: 1, b: 6 },
        Edge { a: 2, b: 5 },
        Edge { a: 2, b: 7 },
        Edge { a: 3, b: 6 },
        Edge { a: 3, b: 4 },
        Edge { a: 0, b: 7 },
        Edge { a: 4, b: 6 },
        Edge { a: 5, b: 7 },
    ],
};

pub const DOUGHNUT: Shape = Shape {
    // How I'm getting values, https://www.desmos.com/calculator/gwpwa2315x
    verts: &[
        // Inner Ring
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 40.0,
        },
        Point3D {
            x: 28.3,
            y: 0.0,
            z: 28.3,
        },
        Point3D {
            x: 40.0,
            y: 0.0,
            z: 0.0,
        },
        Point3D {
            x: 28.3,
            y: 0.0,
            z: -28.3,
        },
        Point3D {
            x: 0.0,
            y: 0.0,
            z: -40.0,
        },
        Point3D {
            x: -28.3,
            y: 0.0,
            z: -28.3,
        },
        Point3D {
            x: -40.0,
            y: 0.0,
            z: 0.0,
        },
        Point3D {
            x: -28.3,
            y: 0.0,
            z: 28.3,
        },
        // Outer Ring
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 100.0,
        },
        Point3D {
            x: 70.7,
            y: 0.0,
            z: 70.7,
        },
        Point3D {
            x: 100.0,
            y: 0.0,
            z: 0.0,
        },
        Point3D {
            x: 70.7,
            y: 0.0,
            z: -70.7,
        },
        Point3D {
            x: 0.0,
            y: 0.0,
            z: -100.0,
        },
        Point3D {
            x: -70.7,
            y: 0.0,
            z: -70.7,
        },
        Point3D {
            x: -100.0,
            y: 0.0,
            z: 0.0,
        },
        Point3D {
            x: -70.7,
            y: 0.0,
            z: 70.7,
        },
        // Upper Middle Ring
        Point3D {
            x: 0.0,
            y: 35.0,
            z: 70.0,
        },
        Point3D {
            x: 49.5,
            y: 35.0,
            z: 49.5,
        },
        Point3D {
            x: 70.0,
            y: 35.0,
            z: 0.0,
        },
        Point3D {
            x: 49.5,
            y: 35.0,
            z: -49.5,
        },
        Point3D {
            x: 0.0,
            y: 35.0,
            z: -70.0,
        },
        Point3D {
            x: -49.5,
            y: 35.0,
            z: -49.5,
        },
        Point3D {
            x: -70.0,
            y: 35.0,
            z: 0.0,
        },
        Point3D {
            x: -49.5,
            y: 35.0,
            z: 49.5,
        },
        // Lower Middle Ring
        Point3D {
            x: 0.0,
            y: -35.0,
            z: 70.0,
        },
        Point3D {
            x: 49.5,
            y: -35.0,
            z: 49.5,
        },
        Point3D {
            x: 70.0,
            y: -35.0,
            z: 0.0,
        },
        Point3D {
            x: 49.5,
            y: -35.0,
            z: -49.5,
        },
        Point3D {
            x: 0.0,
            y: -35.0,
            z: -70.0,
        },
        Point3D {
            x: -49.5,
            y: -35.0,
            z: -49.5,
        },
        Point3D {
            x: -70.0,
            y: -35.0,
            z: 0.0,
        },
        Point3D {
            x: -49.5,
            y: -35.0,
            z: 49.5,
        },
    ],
    edges: &[
        // Inner Ring
        Edge { a: 0, b: 1 },
        Edge { a: 1, b: 2 },
        Edge { a: 2, b: 3 },
        Edge { a: 3, b: 4 },
        Edge { a: 4, b: 5 },
        Edge { a: 5, b: 6 },
        Edge { a: 6, b: 7 },
        Edge { a: 7, b: 0 },
        // Outer Ring
        Edge { a: 8, b: 9 },
        Edge { a: 9, b: 10 },
        Edge { a: 10, b: 11 },
        Edge { a: 11, b: 12 },
        Edge { a: 12, b: 13 },
        Edge { a: 13, b: 14 },
        Edge { a: 14, b: 15 },
        Edge { a: 15, b: 8 },
        // Upper Middle Ring
        Edge { a: 16, b: 17 },
        Edge { a: 17, b: 18 },
        Edge { a: 18, b: 19 },
        Edge { a: 19, b: 20 },
        Edge { a: 20, b: 21 },
        Edge { a: 21, b: 22 },
        Edge { a: 22, b: 23 },
        Edge { a: 23, b: 16 },
        // Lower Middle Ring
        Edge { a: 24, b: 25 },
        Edge { a: 25, b: 26 },
        Edge { a: 26, b: 27 },
        Edge { a: 27, b: 28 },
        Edge { a: 28, b: 29 },
        Edge { a: 29, b: 30 },
        Edge { a: 30, b: 31 },
        Edge { a: 31, b: 24 },
        // Joining Rings
        Edge { a: 0, b: 16 },
        Edge { a: 16, b: 8 },
        Edge { a: 8, b: 24 },
        Edge { a: 24, b: 0 },
        Edge { a: 1, b: 17 },
        Edge { a: 17, b: 9 },
        Edge { a: 9, b: 25 },
        Edge { a: 25, b: 1 },
        Edge { a: 2, b: 18 },
        Edge { a: 18, b: 10 },
        Edge { a: 10, b: 26 },
        Edge { a: 26, b: 2 },
        Edge { a: 3, b: 19 },
        Edge { a: 19, b: 11 },
        Edge { a: 11, b: 27 },
        Edge { a: 27, b: 3 },
        Edge { a: 4, b: 20 },
        Edge { a: 20, b: 12 },
        Edge { a: 12, b: 28 },
        Edge { a: 28, b: 4 },
        Edge { a: 5, b: 21 },
        Edge { a: 21, b: 13 },
        Edge { a: 13, b: 29 },
        Edge { a: 29, b: 5 },
        Edge { a: 6, b: 22 },
        Edge { a: 22, b: 14 },
        Edge { a: 14, b: 30 },
        Edge { a: 30, b: 6 },
        Edge { a: 7, b: 23 },
        Edge { a: 23, b: 15 },
        Edge { a: 15, b: 31 },
        Edge { a: 31, b: 7 },
    ],
};
