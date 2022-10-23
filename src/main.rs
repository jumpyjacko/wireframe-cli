fn main() {
    // Size as in width or height, will always be in a square ratio
    const SIZE: usize = 3;

    let test: Vec<u8> = vec!(1, 1, 1, 0, 0, 0, 1, 1, 1);

    // TODO: Implement modifying and pushing to a vec
    // TODO: Make a vertex and edge table variable
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
            print!("{:?}", test[i]);
        } else {
            print!("{:?}\n", test[i])
        }
    }
}
