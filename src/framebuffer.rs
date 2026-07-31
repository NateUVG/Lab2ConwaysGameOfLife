#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub fn as_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }

    fn from_u32(valor: u32) -> Color {
        Color {
            r: ((valor >> 16) & 0xFF) as u8,
            g: ((valor >> 8) & 0xFF) as u8,
            b: (valor & 0xFF) as u8,
        }
    }
}

pub struct FrameBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    color: Color,
    background: Color,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize, background: Color) -> Self {
        FrameBuffer {
            width,
            height,
            buffer: vec![background.as_u32(); width * height],
            color: Color::WHITE,
            background,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.buffer[y as usize * self.width + x as usize] = self.color.as_u32();
        }
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return self.background;
        }
        Color::from_u32(self.buffer[y as usize * self.width + x as usize])
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
