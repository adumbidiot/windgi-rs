use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::COLORREF;
use winapi::shared::windef::RECT;
use winapi::um::wingdi::RGB;
use winapi::um::wingdi::SRCCOPY;

/// A Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    /// Make a new rect from the given x, y, w, and h values.
    #[inline]
    pub fn new_xywh(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

impl Into<RECT> for Rect {
    #[inline]
    fn into(self) -> RECT {
        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;

        RECT {
            left: x,
            top: y,
            right: x + width,
            bottom: y + height,
        }
    }
}

/// An RGB Color
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Make a new color from the given r, g and b values.
    #[inline]
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl From<COLORREF> for Color {
    fn from(color: COLORREF) -> Color {
        let r = (color & 0xFF) as u8;
        let g = ((color >> 1) & 0xFF) as u8;
        let b = ((color >> 2) & 0xFF) as u8;

        Color::new_rgb(r, g, b)
    }
}

impl Into<COLORREF> for Color {
    fn into(self) -> COLORREF {
        RGB(self.r, self.g, self.b)
    }
}

bitflags::bitflags! {
    pub struct RasterOperation: DWORD {
        /// Copy from the source DeviceContext
        const SRC_COPY = SRCCOPY;
    }
}

impl Into<DWORD> for RasterOperation {
    fn into(self) -> DWORD {
        self.bits()
    }
}
