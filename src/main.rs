struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct Edge {
    a: usize,
    b: usize,
}

fn main() {
    // Size as in width or height, will always be in a square ratio
    const SIZE: usize = 3;

    let screen: Vec<u8> = vec!();

    // TODO: Make a vertex and edge table variable
    // This is a the vert_table and edge_table for a cube
    let vert_table: Vec<Point> = vec!(
        Point { x: -4, y: 4, z: 4 },
        Point { x: 4, y: 4, z: 4 },
        Point { x: 4, y: -4, z: 4 },
        Point { x: -4, y: -4, z: 4 },
        Point { x: -4, y: 4, z: -4 },
        Point { x: 4, y: 4, z: -4 },
        Point { x: 4, y: -4, z: -4 },
        Point { x: -4, y: -4, z: -4 },
    );

    // Assign edges by the index of the verts in the vert_table
    let edge_table: Vec<usize> = vec!(
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

    // TODO: Implement modifying and pushing to a vec
    // TODO: Project verticies
    // TODO: Find equation of a line between two points
    //          Need to find a way to restrict domain?
    // TODO: Implement matrix rotation
    //          If I'm lazy, do the 2D matrix rotation, else do the full rotation matrix
    //          This needs a theta variable as well
    //          Probably watch 3b1b linear algebra series

    // "Render" 1D array as a 2D array
    for i in 0..SIZE*SIZE {
        if (i+1) % SIZE != 0 {
            print!("{:?}", if screen[i] == 1 {"##"} else {"  "});
        } else {
            print!("{:?}\n", if screen[i] == 1 {"##"} else {"  "})
        }
    }
}
