#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
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
        Point3D { x: -50, y: 50, z: 50 },
        Point3D { x: 50, y: 50, z: 50 },
        Point3D { x: 50, y: -50, z: 50 },
        Point3D { x: -50, y: -50, z: 50 },
        Point3D { x: -50, y: 50, z: -50 },
        Point3D { x: 50, y: 50, z: -50 },
        Point3D { x: 50, y: -50, z: -50 },
        Point3D { x: -50, y: -50, z: -50 },
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
    //     Point3D { x: -50, y: -50, z: -50 },
    //     Point3D { x: 50, y: -50, z: -50 },
    //     Point3D { x: 50, y: -50, z: 50 },
    //     Point3D { x: -50, y: -50, z: 50 },
    //     Point3D { x: 0, y: 50, z: 0 },
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

    let mut projected_vert_table: Vec<Point2D> = vec!();
    
    fn project_point(point: &Point3D, focal_length: &i32) -> Point2D {
        let p_x: i32 = (focal_length * point.x) / (focal_length + point.z + 256);
        let p_y: i32 = (focal_length * point.y) / (focal_length + point.z + 256);

        let projected_point: Point2D = Point2D { x: p_x, y: p_y };

        println!("{:?}", projected_point);
        return projected_point;
    }

    fn plot_line(point1: Point2D, point2: Point2D, mut screen: [[u8; SIZE]; SIZE]) -> [[u8; SIZE]; SIZE] {
        let mut dx = point2.x - point1.x;
        let mut dy = point2.y - point1.y;
        // let mut d = 2*dy - dx;
        // let mut y = point1.y;

        let offset = SIZE as i32/2;

        // Bresenham's Line Algorithm (integer arithmetic version)
        // for x in if point1.x < point2.x {point1.x..point2.x} else {point2.x..point1.x} {
        //     screen[(y + offset) as usize][(x + offset) as usize] = 1;

        //     if d > 0 {
        //         y = y+1;
        //         d = d-2*dx;
        //     };
        //     d = d+2*dy;
        // };

        // Naive Line Algorithm
        // for x in if point1.x < point2.x {point1.x..point2.x} else {point2.x..point1.x} {
        //     let y = point1.y + dy * (x - point1.x) / dx;
        //     screen[(y + offset) as usize][(x + offset) as usize] = 1;
        // }

        // Digital Differential Analyzer
        let step;
        if dx.abs() >= dy.abs() {
            step = dx.abs();
        } else {
            step = dy.abs();
        }
        dx = dx / step;
        dy = dy / step;
        let mut x = point1.x;
        let mut y = point1.y;
        let mut i = 1;
        while i <= step {
            screen[(y + offset) as usize][(x + offset) as usize] = 1;
            x = x + dx;
            y = y + dy;
            i += 1;
        }

        return screen;
    }

    // Iterates through vert_table and projects them
    for vert in vert_table.iter() {
        let p_point = project_point(vert, &FOCAL_LENGTH);
        projected_vert_table.push(p_point);
    }

    for edge in edge_table.iter() {
        // println!("({}, {}) ({}, {})", projected_vert_table[edge.a].x, projected_vert_table[edge.a].y, projected_vert_table[edge.b].x, projected_vert_table[edge.b].y);
        screen = plot_line(Point2D { x: projected_vert_table[edge.a].x, y: projected_vert_table[edge.a].y }, Point2D { x: projected_vert_table[edge.b].x, y: projected_vert_table[edge.b].y }, screen);
    }

    println!("{:?}", projected_vert_table);
    // println!("{:?}", edge_table);

    // TODO: Implement matrix rotation
    //          If I'm lazy, do the 2D matrix rotation, else do the full rotation matrix
    //          This needs a theta variable as well
    //          Probably watch 3b1b linear algebra series

    // "Render" 1D array as a 2D array
    for row in screen.iter_mut() {
        for cell in row.iter_mut() {
            // modify cell by *cell = int;
            print!("{}", if *cell as u8 == 1 {"##"} else {".."});
        }
        print!("\n");
    }
}