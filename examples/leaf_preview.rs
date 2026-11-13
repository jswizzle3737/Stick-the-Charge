//! Prints the maple leaf as text, so its outline and the triangulation can be checked
//! without opening a picture.
//!
//! Run: cargo run --example leaf_preview

#[path = "../src/leaf.rs"]
mod leaf;

use eframe::egui::{pos2, Color32, Rect, Shape};

fn main() {
    let cols = 60;
    let rows = 60;
    let rect = Rect::from_min_size(pos2(0.0, 0.0), eframe::egui::vec2(cols as f32, rows as f32));
    let shapes = leaf::leaf_shapes(rect, Color32::WHITE);

    // Collect the triangles the triangulation produced.
    let mut tris: Vec<[eframe::egui::Pos2; 3]> = Vec::new();
    for shape in &shapes {
        if let Shape::Mesh(mesh) = shape {
            for idx in mesh.indices.chunks(3) {
                tris.push([
                    mesh.vertices[idx[0] as usize].pos,
                    mesh.vertices[idx[1] as usize].pos,
                    mesh.vertices[idx[2] as usize].pos,
                ]);
            }
        }
    }

    // Signed area covered by the triangles, for comparison with the polygon's own area.
    let tri_area: f32 = tris
        .iter()
        .map(|[a, b, c]| ((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)).abs() * 0.5)
        .sum();

    let mut covered = 0;
    for row in 0..rows {
        let mut line = String::new();
        for col in 0..cols {
            let p = pos2(col as f32 + 0.5, row as f32 + 0.5);
            let hit = tris.iter().any(|[a, b, c]| inside(p, *a, *b, *c));
            if hit {
                covered += 1;
            }
            line.push(if hit { '#' } else { '.' });
        }
        println!("{line}");
    }
    println!(
        "\ntriangles: {}, triangle area: {:.1} cells, painted cells: {}",
        tris.len(),
        tri_area,
        covered
    );
}

fn inside(p: eframe::egui::Pos2, a: eframe::egui::Pos2, b: eframe::egui::Pos2, c: eframe::egui::Pos2) -> bool {
    let s = |p: eframe::egui::Pos2, a: eframe::egui::Pos2, b: eframe::egui::Pos2| {
        (p.x - b.x) * (a.y - b.y) - (a.x - b.x) * (p.y - b.y)
    };
    let (d1, d2, d3) = (s(p, a, b), s(p, b, c), s(p, c, a));
    let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(neg && pos)
}
