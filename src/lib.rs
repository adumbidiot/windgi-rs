pub mod primitives;

use crate::primitives::Rect;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::COLORREF;
use winapi::shared::windef::HBITMAP;
use winapi::shared::windef::HBRUSH;
use winapi::shared::windef::HDC;
use winapi::shared::windef::HGDIOBJ;
use winapi::shared::windef::HWND;
use winapi::shared::windef::RECT;
use winapi::um::wingdi::BitBlt;
use winapi::um::wingdi::CreateBitmap;
use winapi::um::wingdi::CreateCompatibleDC;
use winapi::um::wingdi::CreateSolidBrush;
use winapi::um::wingdi::DeleteDC;
use winapi::um::wingdi::DeleteObject;
use winapi::um::wingdi::GetDeviceCaps;
use winapi::um::wingdi::GetObjectW;
use winapi::um::wingdi::SelectObject;
use winapi::um::wingdi::StretchBlt;
use winapi::um::wingdi::BITMAP;
use winapi::um::wingdi::HORZRES;
use winapi::um::wingdi::VERTRES;
use winapi::um::winuser::FillRect;
use winapi::um::winuser::GetDC;
use winapi::um::winuser::ReleaseDC;

/// A Device Context
pub struct DeviceContext {
    hwnd: Option<HWND>,
    hdc: HDC,
}

impl DeviceContext {
    /// # Safety
    /// `hwnd` must be a valid `HWND`, or null.
    pub unsafe fn from_hwnd(hwnd: HWND) -> Option<Self> {
        let hdc = GetDC(hwnd);

        if hdc.is_null() {
            None
        } else {
            Some(Self {
                hwnd: Some(hwnd),
                hdc,
            })
        }
    }

    /// Get the desktop
    pub fn desktop() -> Option<Self> {
        unsafe { Self::from_hwnd(std::ptr::null_mut()) }
    }

    /// Get a DeviceContext compatible with this one.
    pub fn get_compatible(&self) -> Option<Self> {
        let hdc = unsafe { CreateCompatibleDC(self.hdc) };
        if hdc.is_null() {
            return None;
        }

        Some(DeviceContext { hwnd: None, hdc })
    }

    /// Returns the raw HDC
    pub fn get_raw_hdc(&self) -> HDC {
        self.hdc
    }

    /// Select a graphics object in this context.
    /// TODO: Wrap return somehow
    pub fn select_object<T: Into<GdiObject>>(&self, object: T) -> *mut c_void {
        unsafe { SelectObject(self.hdc, object.into().0) }
    }

    /// Blits an HDC
    #[allow(clippy::too_many_arguments)]
    pub fn bit_blit(
        &self,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        src: &Self,
        x1: i32,
        x2: i32,
        rop: impl Into<DWORD>,
    ) -> std::io::Result<()> {
        let ret = unsafe { BitBlt(self.hdc, x, y, cx, cy, src.hdc, x1, x2, rop.into()) };

        if ret == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    /// Blits an HDC with stretching
    pub fn stretch_blit(
        &self,
        dest_rect: Rect,
        src: &Self,
        src_rect: Rect,
        rop: impl Into<DWORD>,
    ) -> std::io::Result<()> {
        let ret = unsafe {
            StretchBlt(
                self.hdc,
                dest_rect.x,
                dest_rect.y,
                dest_rect.width,
                dest_rect.height,
                src.hdc,
                src_rect.x,
                src_rect.y,
                src_rect.width,
                src_rect.height,
                rop.into(),
            )
        };

        if ret == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    /// Returns true if successful.
    pub fn fill_rect<R: Into<RECT>>(&self, rect: R, brush: &Brush) -> bool {
        let ret = unsafe { FillRect(self.hdc, &rect.into(), brush.get_raw()) };
        ret != 0
    }

    /// Get the width of this DeviceContext in physical pixels.
    pub fn physical_width(&self) -> i32 {
        unsafe { GetDeviceCaps(self.hdc, HORZRES) }
    }

    /// Get the  height of this DeviceContext in physical pixels.
    pub fn physical_height(&self) -> i32 {
        unsafe { GetDeviceCaps(self.hdc, VERTRES) }
    }
}

impl Drop for DeviceContext {
    fn drop(&mut self) {
        let _ok = if let Some(hwnd) = self.hwnd {
            unsafe { ReleaseDC(hwnd, self.hdc) }
        } else {
            unsafe { DeleteDC(self.hdc) }
        };
    }
}

/// A Brush
#[repr(transparent)]
pub struct Brush(HBRUSH);

impl Brush {
    /// Make a solid brush.
    #[inline]
    pub fn create_solid_brush<T: Into<COLORREF>>(color: T) -> Option<Self> {
        let color = color.into();

        let handle = unsafe { CreateSolidBrush(color) };

        if handle.is_null() {
            None
        } else {
            Some(Brush(handle))
        }
    }

    /// Get the raw inner value.
    #[inline]
    pub fn get_raw(&self) -> HBRUSH {
        self.0
    }
}

impl Drop for Brush {
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.0 as *mut c_void);
        }
    }
}

pub struct GdiObject(HGDIOBJ);

impl Drop for GdiObject {
    fn drop(&mut self) {
        let _deleted = unsafe { DeleteObject(self.0 as *mut c_void) };
    }
}

impl From<BitmapHandle> for GdiObject {
    fn from(handle: BitmapHandle) -> Self {
        handle.0
    }
}

/// A Handle to a bitmap
#[repr(transparent)]
pub struct BitmapHandle(GdiObject);

impl BitmapHandle {
    /// Make a bitmap from the given data.
    pub fn create(
        width: i32,
        height: i32,
        planes: u32,
        bits_per_pixel: u32,
        data: &[u8],
    ) -> Option<Self> {
        let handle = unsafe {
            CreateBitmap(
                width,
                height,
                planes,
                bits_per_pixel,
                data.as_ptr() as *const c_void,
            )
        };

        if handle.is_null() {
            return None;
        }

        // TODO:
        // Handle return code ERROR_INVALID_BITMAP

        Some(BitmapHandle(GdiObject(handle as HGDIOBJ)))
    }

    /// Return a raw bitmap handle
    pub fn get_raw(&self) -> HBITMAP {
        (self.0).0 as HBITMAP
    }

    /// Return the dimensions of this bitmap
    pub fn get_dimensions(&self) -> Option<(i32, i32)> {
        let mut bitmap: BITMAP = unsafe { std::mem::zeroed() };
        let ret = unsafe {
            GetObjectW(
                (self.0).0,
                std::mem::size_of::<BITMAP>() as i32,
                &mut bitmap as *mut BITMAP as *mut c_void,
            )
        };

        if ret == 0 {
            return None;
        }

        Some((bitmap.bmWidth, bitmap.bmHeight))
    }
}
