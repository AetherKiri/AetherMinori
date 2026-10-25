//! Minimal text rasteriser for the headless acceptance backend.
//!
//! The runtime itself does not draw glyphs: on the Aether host the engine emits
//! `minori_message` platform requests and the Godot layer renders them. The
//! headless backend has no Godot, so this module rasterises the same strings
//! into the RGBA frame with a system font, which is what makes dialogue
//! verifiable in a screenshot.

use std::path::Path;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont, point};

pub struct TextRenderer {
    font: FontVec,
}

/// A font that has both Latin and CJK coverage. The retail build renders the
/// Chinese and Japanese scripts, so a Latin-only font is not enough.
const CANDIDATES: &[&str] = &[
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/opentype/noto/NotoSerifCJK-Regular.ttc",
    "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
];

impl TextRenderer {
    pub fn system() -> Result<Self, String> {
        for path in CANDIDATES {
            if !Path::new(path).is_file() {
                continue;
            }
            let data = std::fs::read(path).map_err(|e| format!("read {path}: {e}"))?;
            // A .ttc holds several faces; try each until one parses.
            let count = ttf_parser::fonts_in_collection(&data).unwrap_or(1);
            for index in 0..count {
                if let Ok(font) = FontVec::try_from_vec_and_index(data.clone(), index) {
                    if supports_acceptance_text(&font) {
                        return Ok(Self { font });
                    }
                }
            }
            if let Ok(font) = FontVec::try_from_vec(data) {
                if supports_acceptance_text(&font) {
                    return Ok(Self { font });
                }
            }
        }
        Err("no system font with Latin, Chinese and Japanese glyph coverage found".into())
    }

    /// Draw `text` with word wrapping, clipped to `max_width`.
    ///
    /// Returns the y coordinate just past the last drawn line.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        rgba: &mut [u8],
        width: u32,
        height: u32,
        x: f32,
        y: f32,
        max_width: f32,
        size: f32,
        color: [u8; 4],
        text: &str,
    ) -> f32 {
        let scale = PxScale::from(size);
        let scaled = self.font.as_scaled(scale);
        let line_height = scaled.height() + scaled.line_gap();
        let mut pen_y = y;
        for line in wrap(&self.font, scale, max_width, text) {
            if pen_y + line_height > height as f32 {
                break;
            }
            let mut pen_x = x;
            let mut previous: Option<ab_glyph::GlyphId> = None;
            for ch in line.chars() {
                let glyph_id = scaled.glyph_id(ch);
                if let Some(prev) = previous {
                    pen_x += scaled.kern(prev, glyph_id);
                }
                let glyph = glyph_id.with_scale_and_position(scale, point(pen_x, pen_y));
                if let Some(outlined) = self.font.outline_glyph(glyph) {
                    let bounds = outlined.px_bounds();
                    outlined.draw(|gx, gy, coverage| {
                        let px = bounds.min.x as i32 + gx as i32;
                        let py = bounds.min.y as i32 + gy as i32;
                        if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                            return;
                        }
                        let index = ((py as u32 * width + px as u32) * 4) as usize;
                        blend(&mut rgba[index..index + 4], color, coverage);
                    });
                }
                pen_x += scaled.h_advance(glyph_id);
                previous = Some(glyph_id);
            }
            pen_y += line_height;
        }
        pen_y
    }
}

fn supports_acceptance_text(font: &FontVec) -> bool {
    has_acceptance_glyphs(|ch| font.glyph_id(ch).0 != 0)
}

fn has_acceptance_glyphs(mut mapped: impl FnMut(char) -> bool) -> bool {
    // Representative mappings, not proof of coverage for every script character.
    "A你我日あア。".chars().all(&mut mapped)
}

#[cfg(test)]
mod tests {
    use super::has_acceptance_glyphs;

    #[test]
    fn latin_only_fonts_cannot_pass_dialogue_acceptance() {
        assert!(!has_acceptance_glyphs(|ch| ch.is_ascii()));
        assert!(!has_acceptance_glyphs(|ch| ch != '你'));
        assert!(!has_acceptance_glyphs(|ch| ch != 'あ'));
        assert!(!has_acceptance_glyphs(|ch| ch != 'ア'));
        assert!(has_acceptance_glyphs(|_| true));
    }
}

fn wrap(font: &FontVec, scale: PxScale, max_width: f32, text: &str) -> Vec<String> {
    let scaled = font.as_scaled(scale);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        let mut width = 0.0f32;
        for ch in paragraph.chars() {
            let advance = scaled.h_advance(scaled.glyph_id(ch));
            if width + advance > max_width && !line.is_empty() {
                lines.push(std::mem::take(&mut line));
                width = 0.0;
            }
            line.push(ch);
            width += advance;
        }
        lines.push(line);
    }
    lines
}

fn blend(target: &mut [u8], color: [u8; 4], coverage: f32) {
    let alpha = (coverage * (f32::from(color[3]) / 255.0)).clamp(0.0, 1.0);
    if alpha <= 0.0 {
        return;
    }
    for channel in 0..3 {
        let source = f32::from(color[channel]) * alpha;
        let base = f32::from(target[channel]) * (1.0 - alpha);
        target[channel] = (source + base).round().clamp(0.0, 255.0) as u8;
    }
    target[3] = 255;
}
