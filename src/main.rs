use ndarray::arr1;
use ndarray::arr2;
use clap::{Arg, Command};
use std::{thread, time::Duration, time::Instant};

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

struct Point2D {
    x: i32,
    y: i32,
}

struct Edge {
    a: usize,
    b: usize,
}

fn main() {
    // Size as in width or height, will always be in a square ratio
    const SIZE: usize = 40;
    const FOCAL_LENGTH: i32 = 40;

    let matches = Command::new("wireframe-cli")
        .version("0.3.0")
        .author("Jackson Ly (JumpyJacko)")
        .about("A small wireframe renderer")
        .arg(Arg::new("shape")
            .short('s')
            .long("shape")
            .default_value("cube")
            .help("Pick what shape you want to see (only the ones below)\n    (cube, pyramid, star_cube)"))
        .arg(Arg::new("fill")
            .short('f')
            .long("fill")
            .default_value(".")
            .help("Pick characters to fill whitespace\n    (use single of that character, i.e. '.')"))
        .arg(Arg::new("line")
            .short('l')
            .long("line")
            .default_value("#")
            .help("Pick characters to use for the lines\n    (use single of that character, i.e. '#')"))
        .get_matches();

    
    let fill_char: &str = matches.get_one::<String>("fill").unwrap();
    let line_char: &str = matches.get_one::<String>("line").unwrap();
    let shape: &str = matches.get_one::<String>("shape").unwrap();

    let vert_table: Vec<Point3D>;
    let edge_table: Vec<Edge>;

    match shape {
        "cube" => {
            // NOTE: This is a the vert_table and edge_table for a cube
            vert_table = vec!(
                Point3D { x: -60.0, y: 60.0, z: 60.0 },
                Point3D { x: 60.0, y: 60.0, z: 60.0 },
                Point3D { x: 60.0, y: -60.0, z: 60.0 },
                Point3D { x: -60.0, y: -60.0, z: 60.0 },
                Point3D { x: -60.0, y: 60.0, z: -60.0 },
                Point3D { x: 60.0, y: 60.0, z: -60.0 },
                Point3D { x: 60.0, y: -60.0, z: -60.0 },
                Point3D { x: -60.0, y: -60.0, z: -60.0 },
            );
            edge_table = vec!(
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
        },
        "pyramid" => {
            // NOTE: This is a the vert_table and edge_table for a square-based pyramid
            vert_table = vec!(
                Point3D { x: -60.0, y: -60.0, z: -60.0 },
                Point3D { x: 60.0, y: -60.0, z: -60.0 },
                Point3D { x: 60.0, y: -60.0, z: 60.0 },
                Point3D { x: -60.0, y: -60.0, z: 60.0 },
                Point3D { x: 0.0, y: 60.0, z: 0.0 },
            );
            edge_table = vec!(
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
            );
        },
        "star_cube" => {
            // NOTE: This is a the vert_table and edge_table for a cube with a
            //       different edge table and looks like a star 
            vert_table = vec!(
                Point3D { x: -70.0, y: 70.0, z: 70.0 },
                Point3D { x: 70.0, y: 70.0, z: 70.0 },
                Point3D { x: 70.0, y: -70.0, z: 70.0 },
                Point3D { x: -70.0, y: -70.0, z: 70.0 },
                Point3D { x: -70.0, y: 70.0, z: -70.0 },
                Point3D { x: 70.0, y: 70.0, z: -70.0 },
                Point3D { x: 70.0, y: -70.0, z: -70.0 },
                Point3D { x: -70.0, y: -70.0, z: -70.0 },
            );
            edge_table = vec!(
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
            );
        },
        _ => {
            vert_table = vec!(Point3D { x: 1.0, y: 1.0, z: 1.0 });
            edge_table = vec!(Edge { a:0, b:0 });
            println!("Shape does not exist");
        },
    }
    
    fn project_point(point: &Point3D, focal_length: &i32) -> Point2D {
        let p_x: i32 = ((*focal_length as f32 * point.x) / (*focal_length as f32 + point.z + 256.0)) as i32;
        let p_y: i32 = ((*focal_length as f32 * point.y) / (*focal_length as f32 + point.z + 256.0)) as i32;

        let projected_point: Point2D = Point2D { x: p_x, y: p_y };

        return projected_point;
    }

    // FIXME: Everything renders upsidedown, probably due to the way I index and then plot
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

    fn simple_rotate_x(point: &Point3D, theta: &f32) -> Point3D {
        let x = point.x as f32;
        let yz = arr1(&[point.y as f32, point.z as f32]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()],
                            [theta.sin(), theta.cos()]]);

        let rotated_yz = matrix.dot(&yz);

        let rotated_xyz = Point3D {x: x, y: rotated_yz[0], z: rotated_yz[1] };

        return rotated_xyz;
    }

    fn simple_rotate_y(point: &Point3D, theta: &f32) -> Point3D {
        let y = point.y as f32;
        let xz = arr1(&[point.x as f32, point.z as f32]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()],
                            [theta.sin(), theta.cos()]]);

        let rotated_xz = matrix.dot(&xz);

        let rotated_xyz = Point3D {x: rotated_xz[0], y: y, z: rotated_xz[1] };

        return rotated_xyz;
    }
    
    fn simple_rotate_z(point: &Point3D, theta: &f32) -> Point3D {
        let z = point.z as f32;
        let xy = arr1(&[point.x as f32, point.y as f32]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()],
                            [theta.sin(), theta.cos()]]);

        let rotated_xy = matrix.dot(&xy);

        let rotated_xyz = Point3D {x: rotated_xy[0], y: rotated_xy[1], z: z };

        return rotated_xyz;
    }


    let mut theta: f32 = 0.0;
    
    loop {
        let timer = Instant::now();
        // Clears screen
        print!("\x1B[2J\x1B[1;1H");

        let mut screen: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

        let mut projected_vert_table: Vec<Point2D> = vec!();
        let mut rotated_vert_table: Vec<Point3D> = vec!();

        // Rotates and moves the 3D points
        for vert in vert_table.iter() {
            let r_point = simple_rotate_x(&vert, &theta);
            let r_r_point = simple_rotate_y(&r_point, &theta);
            let r_r_r_point = simple_rotate_z(&r_r_point, &theta);
            rotated_vert_table.push(r_r_r_point);
        }

        // Iterates through vert_table and adds projected points to another vec
        for vert in rotated_vert_table.iter_mut() {
            let p_point = project_point(vert, &FOCAL_LENGTH);
            projected_vert_table.push(p_point);
        }

        // Draws lines between two points determined by the edge_table
        for edge in edge_table.iter() {
            screen = plot_line(Point2D { x: projected_vert_table[edge.a].x, y: projected_vert_table[edge.a].y }, Point2D { x: projected_vert_table[edge.b].x, y: projected_vert_table[edge.b].y }, screen);
        }

        // "Renders" 2D array from 0 and 1 to '  ' and '##'
        for row in screen.iter_mut() {
            for cell in row.iter_mut() {
                print!("{}{0}", if *cell as u8 == 1 {line_char} else {fill_char});
            }
            print!("\n");
        }

        theta += 0.05;
        if theta == 360.0 {theta = 0.0};

        // Stat. readout
        let duration = timer.elapsed().as_micros();
        println!("frame time: {} μs", duration);
        println!("     theta: {}", theta);

        // Determines how long to hold frame (with lower frame duration, the flashing becomes more rapid)
        thread::sleep(Duration::from_millis(40));
    }
}