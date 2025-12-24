use macroquad::prelude::*;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub label: String,
}

impl Button {
    pub fn new(x: f32, y: f32, h: f32, w: f32, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            w,
            h,
            label: label.into(),
        }
    }

    fn contains(&self, mx: f32, my: f32) -> bool {
        mx >= self.x && mx <= self.x + self.w &&
        my >= self.y && my <= self.y + self.h
    }

    pub fn draw(&self) {
        let (mx, my) = mouse_position();
        let hovered = self.contains(mx, my);

        // Background
        let bg = if hovered {
            Color::from_rgba(80, 80, 120, 255)
        } else {
            Color::from_rgba(60, 60, 60, 255)
        };

        draw_rectangle(self.x, self.y, self.w, self.h, bg);

        // Simple border
        draw_rectangle_lines(self.x, self.y, self.w, self.h, 2.0, WHITE);

        // Centered-ish label
        let font_size = 20.0;
        let text_width = measure_text(&self.label, None, font_size as u16, 1.0).width;
        let tx = self.x + (self.w - text_width) / 2.0;
        let ty = self.y + (self.h + font_size) / 2.0 - 4.0;
        draw_text(&self.label, tx, ty, font_size, WHITE);
    }

    /// Returns true *only on the frame* the left mouse button is pressed while over the button
    pub fn clicked(&self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            return self.contains(mx, my);
        }
        false
    }
}
