use ndarray::arr1;
use ndarray::arr2;

use crate::math::{plot_line, Point2D, Point3D};

use wasm_bindgen::prelude::*;

mod math;
mod shapes;

#[derive(Clone, Copy)]
pub struct Edge {
    a: usize,
    b: usize,
}

// Size as in width or height, will always be in a square ratio
pub const SIZE: usize = 40;
// "Camera" focal length, best to have the same as the canvas size
pub const FOCAL_LENGTH: i32 = 40;

#[wasm_bindgen]
pub fn run(fill_char: &str, line_char: &str, shape: &str, theta: f32) -> String {
    let mut output_string: String = "".to_owned();

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
        "donut" | "doughnut" => {
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
        }
    }

    // Render loop
    let mut state: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    let mut projected_vert_table: Vec<Point2D> = vec![];
    let mut rotated_vert_table: Vec<Point3D> = vec![];

    // Rotates and moves the 3D points
    vert_table.iter().for_each(|v| {
        let rotated_point = v.rotate_x(&theta).rotate_y(&theta).rotate_z(&theta);
        rotated_vert_table.push(rotated_point);
    });

    // Iterates through vert_table and adds projected points to another vec
    rotated_vert_table.iter_mut().for_each(|v| {
        let p_point = v.project(&FOCAL_LENGTH);
        projected_vert_table.push(p_point);
    });

    // Draws lines between two points determined by the edge_table
    edge_table.iter().for_each(|e| {
        state = plot_line(
            Point2D {
                x: projected_vert_table[e.a].x,
                y: projected_vert_table[e.a].y,
            },
            Point2D {
                x: projected_vert_table[e.b].x,
                y: projected_vert_table[e.b].y,
            },
            state,
        );
    });

    // "Renders" 2D array from 0 and 1 to '..' and '##'
    for row in state.iter_mut() {
        for cell in row.iter_mut() {
            output_string.push_str(if *cell == 1 { line_char } else { fill_char });
        }
        output_string.push_str("\n");
    }

    output_string
}
