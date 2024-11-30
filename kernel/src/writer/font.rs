use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

pub const LINE_SPACING: usize = 2;
pub const LETTER_SPACING: usize = 0;
pub const BORDER_PADDING: usize = 1;

pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const BACKUP_CHAR: char = '�';
pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FONT_WEIGHT, CHAR_RASTER_HEIGHT);

pub fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}
