use iced::Color;

#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    pub colors: Vec<Color>,
}

impl Default for Palette {
    fn default() -> Self {
        // Sweetie 16 palette (https://lospec.com/palette-list/sweetie-16)
        Self {
            name: "Sweetie 16".to_string(),
            colors: vec![
                Color::from_rgb8(0x1a, 0x1c, 0x2c), // #1a1c2c
                Color::from_rgb8(0x5d, 0x27, 0x5d), // #5d275d
                Color::from_rgb8(0xb1, 0x3e, 0x53), // #b13e53
                Color::from_rgb8(0xef, 0x7d, 0x57), // #ef7d57
                Color::from_rgb8(0xff, 0xcd, 0x75), // #ffcd75
                Color::from_rgb8(0xa7, 0xf0, 0x70), // #a7f070
                Color::from_rgb8(0x38, 0xb7, 0x64), // #38b764
                Color::from_rgb8(0x25, 0x71, 0x79), // #257179
                Color::from_rgb8(0x29, 0x36, 0x6f), // #29366f
                Color::from_rgb8(0x3b, 0x5d, 0xc9), // #3b5dc9
                Color::from_rgb8(0x41, 0xa6, 0xf6), // #41a6f6
                Color::from_rgb8(0x73, 0xef, 0xf7), // #73eff7
                Color::from_rgb8(0xf4, 0xf4, 0xf4), // #f4f4f4
                Color::from_rgb8(0x94, 0xb0, 0xc2), // #94b0c2
                Color::from_rgb8(0x56, 0x6c, 0x86), // #566c86
                Color::from_rgb8(0x33, 0x3c, 0x57), // #333c57
            ],
        }
    }
}

pub fn fetch_lospec_palette(slug: &str) -> Result<Palette, String> {
    let url = format!("https://lospec.com/palette-list/{}.json", slug);
    let json: serde_json::Value = ureq::get(&url)
        .call()
        .map_err(|e| format!("Failed to fetch palette: {}", e))?
        .into_json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let name = json["name"]
        .as_str()
        .unwrap_or(slug)
        .to_string();

    let colors: Vec<Color> = json["colors"]
        .as_array()
        .ok_or("No colors array in response")?
        .iter()
        .filter_map(|c: &serde_json::Value| {
            let hex = c.as_str()?;
            parse_hex_color(hex)
        })
        .collect();

    Ok(Palette { name, colors })
}

fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::from_rgb8(r, g, b))
}
