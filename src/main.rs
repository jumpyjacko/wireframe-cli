use ndarray::arr1;
use ndarray::arr2;

#[derive(Debug)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Edge {
    a: usize,
    b: usize,
}

fn main() {
    // Size as in width or height, will always be in a square ratio
    const SIZE: usize = 40;
    const FOCAL_LENGTH: i32 = 40;

    // TODO: Render loop which resets screen so I can animate
    let mut screen: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    // FIXME: Everything here renders upside down lol, probably due to the way I index and then plot
    // NOTE: This is a the vert_table and edge_table for a cube
    let vert_table: Vec<Point3D> = vec!(
        Point3D { x: -50.0, y: 50.0, z: 50.0 },
        Point3D { x: 50.0, y: 50.0, z: 50.0 },
        Point3D { x: 50.0, y: -50.0, z: 50.0 },
        Point3D { x: -50.0, y: -50.0, z: 50.0 },
        Point3D { x: -50.0, y: 50.0, z: -50.0 },
        Point3D { x: 50.0, y: 50.0, z: -50.0 },
        Point3D { x: 50.0, y: -50.0, z: -50.0 },
        Point3D { x: -50.0, y: -50.0, z: -50.0 },
    );

    // Assign edges by the index of the verts in the vert_table
    let edge_table: Vec<Edge> = vec!(
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
    );

    // NOTE: This is a the vert_table and edge_table for a square-based pyramid FIXME: This is not working right now
    // let vert_table: Vec<Point3D> = vec!(
    //     Point3D { x: -50.0, y: -50.0, z: -50.0 },
    //     Point3D { x: 50.0, y: -50.0, z: -50.0 },
    //     Point3D { x: 50.0, y: -50.0, z: 50.0 },
    //     Point3D { x: -50.0, y: -50.0, z: 50.0 },
    //     Point3D { x: 0.0, y: 50.0, z: 0.0 },
    // );

    // // Assign edges by the index of the verts in the vert_table
    // let edge_table: Vec<Edge> = vec!(
    //     // Base Face
    //     Edge { a: 0, b: 1 },
    //     Edge { a: 1, b: 2 },
    //     Edge { a: 2, b: 3 },
    //     Edge { a: 3, b: 0 },
        
    //     // Base-to-Point Edges
    //     Edge { a: 0, b: 4 },
    //     Edge { a: 1, b: 4 },
    //     Edge { a: 2, b: 4 },
    //     Edge { a: 3, b: 4 },
    // );

    let mut rotated_vert_table: Vec<Point3D> = vec!();
    let mut projected_vert_table: Vec<Point2D> = vec!();
    
    fn project_point(point: &Point3D, focal_length: &i32) -> Point2D {
        let p_x: i32 = ((*focal_length as f32 * point.x) / (*focal_length as f32 + point.z + 256.0)) as i32;
        let p_y: i32 = ((*focal_length as f32 * point.y) / (*focal_length as f32 + point.z + 256.0)) as i32;

        let projected_point: Point2D = Point2D { x: p_x, y: p_y };

        println!("{:?}", projected_point);
        return projected_point;
    }

    /// Plots a line given 2 points and the screen to modify
    fn plot_line(point1: Point2D, point2: Point2D, mut screen: [[u8; SIZE]; SIZE]) -> [[u8; SIZE]; SIZE] {
        let offset: f32 = SIZE as f32/2.0;

        // Digital Differential Analyzer Line Drawing Algorithm
        let mut dx: f32 = point2.x as f32 - point1.x as f32;
        let mut dy: f32 = point2.y as f32 - point1.y as f32;
        let step;
        if dx.abs() >= dy.abs() {
            step = dx.abs();
        } else {
            step = dy.abs();
        }
        dx = dx / step;
        dy = dy / step;
        let mut x = point1.x as f32;
        let mut y = point1.y as f32;
        let mut i = 1;
        while i as f32 <= step {
            screen[(y + offset) as usize][(x + offset) as usize] = 1;
            x = x + dx;
            y = y + dy;
            i += 1;
        }

        return screen;
    }

    // TODO: Implement matrix rotation
    //          If I'm lazy, do the 2D matrix rotation, else do the full rotation matrix
    //          This needs a theta variable as well
    //          Probably watch 3b1b linear algebra series
    fn simple_rotate_z(point: &Point3D, theta: f32) -> Point3D {
        let z = point.z as f32;
        let xy = arr1(&[point.x as f32, point.y as f32]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()],
                            [theta.sin(), theta.cos()]]);

        let rotated_xy = matrix.dot(&xy);

        let rotated_xyz = Point3D {x: rotated_xy[0], y: rotated_xy[1], z: z };

        return rotated_xyz;
    }


    // Iterates through vert_table and adds projected points to another vec
    for vert in vert_table.iter() {
        let r_point = simple_rotate_z(vert, 90.0);
        rotated_vert_table.push(r_point);
    }

    for vert in rotated_vert_table.iter() {
        let p_point = project_point(vert, &FOCAL_LENGTH);
        projected_vert_table.push(p_point);
    }

    for edge in edge_table.iter() {
        screen = plot_line(Point2D { x: projected_vert_table[edge.a].x, y: projected_vert_table[edge.a].y }, Point2D { x: projected_vert_table[edge.b].x, y: projected_vert_table[edge.b].y }, screen);
    }

    // "Renders" 2D array from 0 and 1 to '  ' and '##'
    for row in screen.iter_mut() {
        for cell in row.iter_mut() {
            print!("{}", if *cell as u8 == 1 {"##"} else {".."});
        }
        print!("\n");
    }
}