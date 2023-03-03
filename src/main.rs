extern crate bdf;
use std::{env, fs::File, io::Write};

use crate::preview::preview_glyph;

mod preview;

fn main() {
    let font = bdf::open(env::args().nth(1).expect("Missing font file")).unwrap();
    let glyphs = font.glyphs();

    let mut file = File::create("font.bin").expect("Cannot create file");

    for (_, glyph) in glyphs {
        let mut binary: Vec<u8> = Vec::new();

        for y in 0..16 {
            let byte_left = u8::from(glyph.get(0, y)) << 7
                | u8::from(glyph.get(1, y)) << 6
                | u8::from(glyph.get(2, y)) << 5
                | u8::from(glyph.get(3, y)) << 4
                | u8::from(glyph.get(4, y)) << 3
                | u8::from(glyph.get(5, y)) << 2
                | u8::from(glyph.get(6, y)) << 1
                | u8::from(glyph.get(7, y));

            binary.push(byte_left);

            let byte_right = u8::from(glyph.get(8, y)) << 7
                | u8::from(glyph.get(9, y)) << 6
                | u8::from(glyph.get(10, y)) << 5
                | u8::from(glyph.get(11, y)) << 4
                | u8::from(glyph.get(12, y)) << 3
                | u8::from(glyph.get(13, y)) << 2
                | u8::from(glyph.get(14, y)) << 1
                | u8::from(glyph.get(15, y));

            binary.push(byte_right);
        }

        preview_glyph(glyph);

        // 16x16 のタイルの場合、視覚的に正方形のグラフィックデータを 8×8 ずつで 4つに分割し、以下のように並んでいると考えると、
        // AB
        // CD
        // メモリ上の配置としては、A ブロック → B ブロックが隣接して並び、0100H 空けて C ブロック → D ブロックの順で並ぶ

        let remapped_binary = [
            binary[0], binary[2], binary[4], binary[6], binary[8], binary[10], binary[12],
            binary[14], // Aブロック
            binary[1], binary[3], binary[5], binary[7], binary[9], binary[11], binary[13],
            binary[15], // Bブロック
            binary[16], binary[18], binary[20], binary[22], binary[24], binary[26], binary[28],
            binary[30], // Cブロック
            binary[17], binary[19], binary[21], binary[23], binary[25], binary[27], binary[29],
            binary[31], // Dブロック
        ];

        file.write_all(&remapped_binary).expect("Cannot write");
    }
}
