#[derive(Clone, Debug, PartialEq)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn r(&self) -> u8 {
        self.0
    }
    pub fn g(&self) -> u8 {
        self.1
    }
    pub fn b(&self) -> u8 {
        self.2
    }
    pub fn a(&self) -> u8 {
        self.3
    }

    pub fn to_rgba_string(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color(value.0, value.1, value.2, 0xFF)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Color(value.0, value.1, value.2, value.3)
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color(value[0], value[1], value[2], 0xFF)
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Color(value[0], value[1], value[2], value[3])
    }
}
