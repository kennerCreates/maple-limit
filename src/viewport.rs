use iced::Point;

pub struct Viewport {
    pub offset: Point,
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            offset: Point::ORIGIN,
            zoom: 1.0,
        }
    }
}

impl Viewport {
    pub fn screen_to_world(&self, screen: Point) -> Point {
        Point::new(
            (screen.x - self.offset.x) / self.zoom,
            (screen.y - self.offset.y) / self.zoom,
        )
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.offset.x += dx;
        self.offset.y += dy;
    }

    pub fn zoom_at(&mut self, cursor_screen: Point, factor: f32) {
        let old_world = self.screen_to_world(cursor_screen);
        self.zoom *= factor;
        self.zoom = self.zoom.clamp(0.1, 10.0);
        // Adjust offset so the world point under cursor stays fixed
        self.offset.x = cursor_screen.x - old_world.x * self.zoom;
        self.offset.y = cursor_screen.y - old_world.y * self.zoom;
    }
}
