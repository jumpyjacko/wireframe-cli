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
    const SIZE: usize = 30;
    const FOCAL_LENGTH: i32 = 3;

    // TODO: Render loop which resets screen
    let mut screen: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    // TODO: Make a vertex and edge table variable
    // This is a the vert_table and edge_table for a cube
    let vert_table: Vec<Point3D> = vec!(
        Point3D { x: -10, y: 10, z: 10 },
        Point3D { x: 10, y: 10, z: 10 },
        Point3D { x: 10, y: -10, z: 10 },
        Point3D { x: -10, y: -10, z: 10 },
        Point3D { x: -10, y: 10, z: -10 },
        Point3D { x: 10, y: 10, z: -10 },
        Point3D { x: 10, y: -10, z: -10 },
        Point3D { x: -10, y: -10, z: -10 },
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
    
    // TODO: Project verticies
    // NOTE: Might have to move from ints to floats
    fn project_point( point: &Point3D, focal_length: &i32) -> Point2D {
        let p_x: i32 = (focal_length * point.x) / (focal_length + point.z);
        let p_y: i32 = (focal_length * point.y) / (focal_length + point.z);

        let projected_point: Point2D = Point2D { x: p_x, y: p_y };

        return projected_point;
    }

    // TODO: Find equation of a line between two points
    //          Need to find a way to restrict domain?
    // TODO: Implement modifying and pushing to a vec, this is the display
    // Uses Bresenham's Line Algorithm (integer arithmetic)
    fn plot_line(point1: Point2D, point2: Point2D, mut screen: [[u8; SIZE]; SIZE]) -> [[u8; SIZE]; SIZE] {
        let dx = point2.x - point1.x;
        let dy = point2.y - point1.x;
        let mut d = 2*dy - dx;
        let mut y = point1.y;

        for x in point1.x..point2.x {
            screen[y as usize][x as usize] = 1;

            if d > 0 {
                y = y+1;
                d = d-2*dx;
            };
            d = d+2*dy;
        };
        return screen;
    }

    // Plots the points into the 2D array
    fn plot(point: Point2D) {

    }

    // TODO: Implement matrix rotation
    //          If I'm lazy, do the 2D matrix rotation, else do the full rotation matrix
    //          This needs a theta variable as well
    //          Probably watch 3b1b linear algebra series

    let a = project_point(&vert_table[0], &FOCAL_LENGTH);

    println!("{:?}", a);

    // "Render" 1D array as a 2D array
    // for i in 0..SIZE*SIZE {
    //     if (i+1) % SIZE != 0 {
    //         print!("{:?}", if screen[i] == 1 {"##"} else {"  "});
    //     } else {
    //         print!("{:?}\n", if screen[i] == 1 {"##"} else {"  "})
    //     }
    // }

    screen = plot_line( Point2D {x: 0, y: 0}, Point2D {x: 3, y: 3}, screen);

    for row in screen.iter_mut() {
        for cell in row.iter_mut() {
            // modify cell by *cell = int;
            print!("{:?}", if *cell as u8 == 1 {"##"} else {"  "});
        }
        print!("\n");
    }
}