use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use nalgebra::Point4;

use crate::mesh_data::MeshData;

pub fn read_object(filename: &str) -> io::Result<MeshData> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut verts: Vec<Point4<f64>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.starts_with("v ") {
                    let tokens: Vec<&str> = l.split(" ").collect();

                    if tokens.len() != 4 {
                        panic!("Line doesn't have four tokens: {}", l)
                    }

                    verts.push(Point4::new(
                        tokens[1].parse::<f64>().unwrap(),
                        tokens[2].parse::<f64>().unwrap(),
                        tokens[3].parse::<f64>().unwrap(),
                        1.0,
                    ));
                }
            }
            Err(_) => todo!(),
        }
    }
    println!("{:?}", verts);

    Ok(MeshData { verts })
}
