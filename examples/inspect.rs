fn main() {
    let path = std::env::args().nth(1).expect("usage: inspect <file.png>");
    let img = image::open(&path).expect("open png").to_rgba8();
    let (w, h) = img.dimensions();
    let mut colors: std::collections::HashSet<[u8; 3]> = std::collections::HashSet::new();
    let mut bright = 0u64;
    let mut accent = 0u64;
    let total = (w as u64) * (h as u64);
    for px in img.pixels() {
        let [r, g, b, _] = px.0;
        colors.insert([r, g, b]);
        if r > 200 && g > 200 && b > 200 {
            bright += 1;
        }
        if r > 150 && g < 80 && b < 90 {
            accent += 1;
        }
    }
    println!("{path}: {w}x{h}, distinct colours {}, bright pixels {:.2}%, maple red {:.2}%",
        colors.len(),
        100.0 * bright as f64 / total as f64,
        100.0 * accent as f64 / total as f64);
}
