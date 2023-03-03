use bdf::Glyph;

pub fn preview_glyph(glyph: &Glyph) {
    // Glyph preview
    for y in 0..glyph.height() {
        for x in 0..glyph.width() {
            if glyph.get(x, y) {
                print!("██");
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }
}
