use iced::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridStyle {
    Lines,
    Dots,
    Isometric,
}

impl GridStyle {
    pub const ALL: &'static [GridStyle] = &[GridStyle::Lines, GridStyle::Dots, GridStyle::Isometric];
}

impl std::fmt::Display for GridStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridStyle::Lines => write!(f, "Lines"),
            GridStyle::Dots => write!(f, "Dots"),
            GridStyle::Isometric => write!(f, "Isometric"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridConfig {
    pub style: GridStyle,
    pub size: f32,
    pub visible: bool,
    pub snap: bool,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            style: GridStyle::Lines,
            size: 20.0,
            visible: true,
            snap: false,
        }
    }
}

pub fn snap_to_grid(point: Point, config: &GridConfig) -> Point {
    match config.style {
        GridStyle::Lines | GridStyle::Dots => {
            let s = config.size;
            Point::new(
                (point.x / s).round() * s,
                (point.y / s).round() * s,
            )
        }
        GridStyle::Isometric => {
            // Rotated 90° CW basis: e1 = (0, s), e2 = (-s*sqrt3_2, s/2)
            let s = config.size;
            let sqrt3_2 = 3.0_f32.sqrt() / 2.0;

            // Solve point = a*e1 + b*e2:
            // x = -b * s * sqrt3_2  =>  b = -x / (s * sqrt3_2)
            // y = a * s + b * s/2   =>  a = (y - b * s / 2) / s
            let b = -point.x / (s * sqrt3_2);
            let a = (point.y - b * s / 2.0) / s;

            let a_r = a.round();
            let b_r = b.round();

            Point::new(
                -b_r * s * sqrt3_2,
                a_r * s + b_r * s / 2.0,
            )
        }
    }
}
