#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winapi::um::shellscalingapi::SetProcessDpiAwareness;
use winapi::um::shellscalingapi::PROCESS_PER_MONITOR_DPI_AWARE;
use windgi::primitives::Color;
use windgi::primitives::Rect;
use windgi::Brush;
use windgi::DeviceContext;

fn main() {
    unsafe {
        SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    }

    let ctx = DeviceContext::desktop().expect("Failed to create DeviceContext");
    let color = Color::new_rgb(0, 0, 255);
    let brush = Brush::create_solid_brush(color).expect("Failed to create Solid Brush");

    loop {
        let ctx_width = ctx.physical_width();
        let ctx_height = ctx.physical_height();

        let rect = Rect::new_xywh(0, 0, ctx_width, ctx_height);

        assert!(ctx.fill_rect(rect, &brush), "Failed to fill rect");
    }
}
