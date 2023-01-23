use clap::{Arg, Command};
use ndarray::arr1;
use ndarray::arr2;
use std::{thread, time::Duration, time::Instant};

use crate::{
    math::{
        simple_rotate_x,
        simple_rotate_y,
        simple_rotate_z},
    render::{plot_line, project_point}
};

mod math;
mod render;
mod shapes;

#[derive(Clone, Copy)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
pub struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
pub struct Edge {
    a: usize,
    b: usize,
}

// Size as in width or height, will always be in a square ratio
pub const SIZE: usize = 40;
// "Camera" focal length, best to have the same as the canvas size
pub const FOCAL_LENGTH: i32 = 40;

fn main() {
    // CLI setup
    let matches = Command::new("wireframe-cli")
        .version("0.4.0")
        .author("Jackson Ly (JumpyJacko)")
        .about("A small wireframe renderer")
        .arg(Arg::new("shape")
            .short('s')
            .long("shape")
            .default_value("cube")
            .help("Pick what shape you want to see (only the ones below)\n    (cube, pyramid, star_cube, donut)"))
        .arg(Arg::new("fill")
            .short('f')
            .long("fill")
            .default_value(".")
            .help("Pick characters to fill whitespace\n    (use only one of that character)"))
        .arg(Arg::new("line")
            .short('l')
            .long("line")
            .default_value("#")
            .help("Pick characters to use for the lines\n    (use only one of that character)"))
        .arg(Arg::new("frame-time")
            .short('t')
            .long("frame-time")
            .default_value("40")
            .help("Input how long to hold a frame\n    (in millis)"))
        .get_matches();

    let fill_char: &str = matches.get_one::<String>("fill").unwrap();
    let line_char: &str = matches.get_one::<String>("line").unwrap();
    let shape: &str = matches.get_one::<String>("shape").unwrap();
    let frame_time = matches
        .get_one::<String>("frame-time")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let vert_table: Vec<Point3D>;
    let edge_table: Vec<Edge>;

    match shape {
        "cube" => {
            vert_table = shapes::CUBE.verts.to_vec();
            edge_table = shapes::CUBE.edges.to_vec();
        }
        "pyramid" => {
            vert_table = shapes::PYRAMID.verts.to_vec();
            edge_table = shapes::PYRAMID.edges.to_vec();
        }
        "star_cube" => {
            vert_table = shapes::STAR_CUBE.verts.to_vec();
            edge_table = shapes::STAR_CUBE.edges.to_vec();
        }
        "donut" => {
            vert_table = shapes::DOUGHNUT.verts.to_vec();
            edge_table = shapes::DOUGHNUT.edges.to_vec();
        }
        _ => {
            vert_table = vec![Point3D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }];
            edge_table = vec![Edge { a: 0, b: 0 }];
            println!("Shape does not exist");
        }
    }

    let mut theta: f32 = 0.0;

    // Initial clear screen
    print!("\x1B[2J\x1B[1;1H");

    // Render loop
    loop {
        let timer = Instant::now();
        // Moves cursor to top left
        print!("\x1B[1;1H");

        let mut screen: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

        let mut projected_vert_table: Vec<Point2D> = vec![];
        let mut rotated_vert_table: Vec<Point3D> = vec![];

        // Rotates and moves the 3D points
        for vert in vert_table.iter() {
            let rotated_point = simple_rotate_z(&simple_rotate_y(&simple_rotate_x(vert, &theta), &theta), &theta);
            rotated_vert_table.push(rotated_point);
        }

        // Iterates through vert_table and adds projected points to another vec
        for vert in rotated_vert_table.iter_mut() {
            let p_point = project_point(vert, &FOCAL_LENGTH);
            projected_vert_table.push(p_point);
        }

        // Draws lines between two points determined by the edge_table
        for edge in edge_table.iter() {
            screen = plot_line(
                Point2D {
                    x: projected_vert_table[edge.a].x,
                    y: projected_vert_table[edge.a].y,
                },
                Point2D {
                    x: projected_vert_table[edge.b].x,
                    y: projected_vert_table[edge.b].y,
                },
                screen,
            );
        }

        // "Renders" 2D array from 0 and 1 to '..' and '##'
        for row in screen.iter_mut() {
            for cell in row.iter_mut() {
                print!(
                    "{}{0}",
                    if *cell == 1 {
                        line_char
                    } else {
                        fill_char
                    }
                );
            }
            println!();
        }

        theta += 0.05;
        if theta == 360.0 {
            theta = 0.0
        };

        // Stat. readout
        let duration = timer.elapsed().as_micros();
        println!("calculation duration: {} μs   ", duration);
        println!("     frame hold time: {} ms   ", frame_time);
        println!("               theta: {}      ", theta);

        // Determines how long to hold frame
        thread::sleep(Duration::from_millis(frame_time));
    }
}
