#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winapi::um::shellscalingapi::SetProcessDpiAwareness;
use winapi::um::shellscalingapi::PROCESS_PER_MONITOR_DPI_AWARE;
use windgi::primitives::RasterOperation;
use windgi::primitives::Rect;
use windgi::BitmapHandle;
use windgi::DeviceContext;

const BRANDON: &[u8] = include_bytes!("./brandon.jpeg");

pub type BgraImage = image::ImageBuffer<image::Bgra<u8>, Vec<u8>>;

fn image_to_bitmap_handle(image: &BgraImage) -> Option<BitmapHandle> {
    let brga8_num_channels = 4;
    let bgra8_bits_per_char = 8;

    BitmapHandle::create(
        image.width() as i32,
        image.height() as i32,
        brga8_num_channels,
        bgra8_bits_per_char,
        &image,
    )
}

fn main() {
    let brandon_image = image::load_from_memory(BRANDON).expect("Failed to load image");

    unsafe {
        SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    }

    let ctx = DeviceContext::desktop().expect("Failed to create DeviceContext");

    let bitmap =
        image_to_bitmap_handle(&brandon_image.into_bgra()).expect("Failed to get bitmap handle");
    let bitmap_dimensions = bitmap
        .get_dimensions()
        .expect("Failed to get bitmap dimensions");

    let bitmap_dc = ctx
        .get_compatible()
        .expect("Failed to get bitmap device context");
    let _hbm_old = bitmap_dc.select_object(bitmap);

    let src_rect = Rect::new_xywh(0, 0, bitmap_dimensions.0, bitmap_dimensions.1);

    loop {
        let ctx_width = ctx.physical_width();
        let ctx_height = ctx.physical_height();

        let dest_rect = Rect::new_xywh(0, 0, ctx_width, ctx_height);

        ctx.stretch_blit(dest_rect, &bitmap_dc, src_rect, RasterOperation::SRC_COPY)
            .expect("Failed to blit image to screen");
    }
}
