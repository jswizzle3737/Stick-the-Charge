//! The maple leaf, drawn from an outline so it stays crisp at any size.
//!
//! Drawn here by hand rather than from a font or a picture, so the app ships no third-party
//! artwork. The outline is a symmetric, 25-point stylised leaf with a short stem.

use eframe::egui::{self, pos2, Color32, Mesh, Pos2, Rect, Shape, Vec2};

/// Outline in model space: origin at the leaf's centre, y up, the leaf spans -1..=1.
/// Listed clockwise from the top tip, down the right side (lobe tip, crease, lobe tip…) and
/// round the stem, then back up the left side.
#[allow(dead_code)]
const OUTLINE: &[(f32, f32)] = &[
    (0.00, 1.00),  // top tip
    (0.16, 0.60),  // crease
    (0.44, 0.74),  // upper lobe tip
    (0.34, 0.38),  // crease
    (0.72, 0.48),  // side lobe tip
    (0.50, 0.12),  // crease
    (0.94, 0.10),  // lower side lobe tip
    (0.58, -0.18), // crease
    (0.50, -0.50), // bottom lobe tip
    (0.19, -0.34), // crease into the stem
    (0.09, -0.38), // stem, top right
    (0.09, -1.00), // stem, bottom right
    (-0.09, -1.00),
    (-0.09, -0.38),
    (-0.19, -0.34),
    (-0.50, -0.50),
    (-0.58, -0.18),
    (-0.94, 0.10),
    (-0.50, 0.12),
    (-0.72, 0.48),
    (-0.34, 0.38),
    (-0.44, 0.74),
    (-0.16, 0.60),
];

/// A leaf's filled triangles, scaled into `rect`.
pub fn leaf_shapes(rect: Rect, color: Color32) -> Vec<Shape> {
    shapes_from_outline(FLAG_OUTLINE, rect, color)
}

/// The Canadian flag leaf, with its rounded corners sampled from the flag geometry.
/// Geometry: https://commons.wikimedia.org/wiki/File:Flag_of_Canada_(leaf).svg
#[allow(dead_code)]
pub fn flag_leaf_shapes(rect: Rect, color: Color32) -> Vec<Shape> {
    shapes_from_outline(FLAG_OUTLINE, rect, color)
}

fn shapes_from_outline(outline: &[(f32, f32)], rect: Rect, color: Color32) -> Vec<Shape> {
    let side = rect.width().min(rect.height()) * 0.5;
    let c = rect.center();
    let points: Vec<Pos2> = outline
        .iter()
        // model y is up, egui's y points down
        .map(|(x, y)| pos2(c.x + x * side, c.y - y * side))
        .collect();

    let tris = triangulate(&points);
    let mut mesh = Mesh::default();
    for p in &points {
        mesh.colored_vertex(*p, color);
    }
    for [a, b, cc] in tris {
        mesh.add_triangle(a, b, cc);
    }
    vec![Shape::mesh(mesh)]
}

const FLAG_OUTLINE: &[(f32, f32)] = &[
    (-0.04432, -1.00000), (-0.02216, -0.57498), (-0.02582, -0.55458),
    (-0.03793, -0.53775), (-0.05613, -0.52782), (-0.07683, -0.52672),
    (-0.49988, -0.60108), (-0.44275, -0.44349), (-0.44079, -0.43377),
    (-0.44190, -0.42393), (-0.44597, -0.41489), (-0.45260, -0.40754),
    (-0.91603, -0.03226), (-0.81162, 0.01650), (-0.80227, 0.02321),
    (-0.79590, 0.03279), (-0.79332, 0.04400), (-0.79488, 0.05541),
    (-0.88648, 0.33711), (-0.61955, 0.28047), (-0.60864, 0.28010),
    (-0.59824, 0.28342), (-0.58955, 0.29003), (-0.58360, 0.29919),
    (-0.53189, 0.42083), (-0.32357, 0.19724), (-0.30571, 0.18771),
    (-0.28571, 0.19079), (-0.27155, 0.20525), (-0.26890, 0.22531),
    (-0.36937, 0.74341), (-0.20832, 0.65033), (-0.19600, 0.64617),
    (-0.18303, 0.64726), (-0.17157, 0.65342), (-0.16351, 0.66363),
    (0.00000, 0.98473), (0.16351, 0.66363), (0.17157, 0.65342),
    (0.18303, 0.64726), (0.19600, 0.64617), (0.20832, 0.65033),
    (0.36937, 0.74341), (0.26890, 0.22531), (0.27155, 0.20525),
    (0.28571, 0.19079), (0.30571, 0.18771), (0.32357, 0.19724),
    (0.53189, 0.42083), (0.58360, 0.29919), (0.58955, 0.29003),
    (0.59824, 0.28342), (0.60864, 0.28010), (0.61955, 0.28047),
    (0.88648, 0.33711), (0.79488, 0.05541), (0.79332, 0.04400),
    (0.79590, 0.03279), (0.80227, 0.02321), (0.81162, 0.01650),
    (0.91603, -0.03226), (0.45260, -0.40754), (0.44597, -0.41489),
    (0.44190, -0.42393), (0.44079, -0.43377), (0.44275, -0.44349),
    (0.49988, -0.60108), (0.07683, -0.52672), (0.05613, -0.52782),
    (0.03793, -0.53775), (0.02582, -0.55458), (0.02216, -0.57498),
    (0.04432, -1.00000),
];

/// A leaf inside a filled circle: the app's mark.
#[allow(dead_code)]
pub fn paint_mark(ui: &mut egui::Ui, diameter: f32, disc: Color32, leaf_color: Color32) -> Rect {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(diameter), egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), diameter * 0.5, disc);
    let inner = Rect::from_center_size(rect.center(), Vec2::splat(diameter * 0.62));
    for shape in leaf_shapes(inner, leaf_color) {
        ui.painter().add(shape);
    }
    rect
}

/// A leaf on its own, no disc behind it.
#[allow(dead_code)]
pub fn paint_leaf(ui: &mut egui::Ui, size: Vec2, color: Color32) -> Rect {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let target = Rect::from_center_size(rect.center(), Vec2::splat(size.y.min(size.x) * 0.96));
    for shape in leaf_shapes(target, color) {
        ui.painter().add(shape);
    }
    rect
}

// --- triangulation ------------------------------------------------------------------

/// Ear clipping, enough for a simple polygon like this one.
fn triangulate(pts: &[Pos2]) -> Vec<[u32; 3]> {
    let n = pts.len();
    if n < 3 {
        return Vec::new();
    }
    let mut area = 0.0;
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        area += a.x * b.y - b.x * a.y;
    }
    let mut idx: Vec<usize> = (0..n).collect();
    if area < 0.0 {
        idx.reverse();
    }

    let mut tris = Vec::new();
    let mut guard = 0;
    while idx.len() > 3 && guard < 4096 {
        guard += 1;
        let m = idx.len();
        let mut clipped = false;
        for i in 0..m {
            let i0 = idx[(i + m - 1) % m];
            let i1 = idx[i];
            let i2 = idx[(i + 1) % m];
            let (a, b, c) = (pts[i0], pts[i1], pts[i2]);
            let cross = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
            if cross <= 0.0001 {
                continue; // reflex corner, not an ear
            }
            let mut blocked = false;
            for &j in idx.iter() {
                if j == i0 || j == i1 || j == i2 {
                    continue;
                }
                if inside_triangle(pts[j], a, b, c) {
                    blocked = true;
                    break;
                }
            }
            if blocked {
                continue;
            }
            tris.push([i0 as u32, i1 as u32, i2 as u32]);
            idx.remove(i);
            clipped = true;
            break;
        }
        if !clipped {
            break;
        }
    }
    if idx.len() == 3 {
        tris.push([idx[0] as u32, idx[1] as u32, idx[2] as u32]);
    }
    tris
}

fn inside_triangle(p: Pos2, a: Pos2, b: Pos2, c: Pos2) -> bool {
    let d1 = (p.x - b.x) * (a.y - b.y) - (a.x - b.x) * (p.y - b.y);
    let d2 = (p.x - c.x) * (b.y - c.y) - (b.x - c.x) * (p.y - c.y);
    let d3 = (p.x - a.x) * (c.y - a.y) - (c.x - a.x) * (p.y - a.y);
    let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(neg && pos)
}
