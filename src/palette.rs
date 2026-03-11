use iced::Color;

#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    pub colors: Vec<Color>,
}

impl Default for Palette {
    fn default() -> Self {
        // Default palette with common colors
        Self {
            name: "Default".to_string(),
            colors: vec![
                Color::BLACK,
                Color::WHITE,
                Color::from_rgb(1.0, 0.0, 0.0),
                Color::from_rgb(0.0, 1.0, 0.0),
                Color::from_rgb(0.0, 0.0, 1.0),
                Color::from_rgb(1.0, 1.0, 0.0),
                Color::from_rgb(1.0, 0.0, 1.0),
                Color::from_rgb(0.0, 1.0, 1.0),
                Color::from_rgb(1.0, 0.5, 0.0),
                Color::from_rgb(0.5, 0.0, 1.0),
                Color::from_rgb(0.5, 0.5, 0.5),
                Color::from_rgb(0.75, 0.75, 0.75),
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
