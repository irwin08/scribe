use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::draw_hollow_rect;
use imageproc::rect::Rect;

pub fn draw_smallest_rect(image: DynamicImage) -> Result<DynamicImage, &'static str> {
    let image_buffer = image.into_rgba8();

    
    const LINE_WIDTH : u32 = 135;
    // find top rectangle line. Should be a straight line that is all white/paper. Find closest to text (so line not all white.) Add some offset. 
    // start in the middle
    let mut top_y_pos = 0;
    
    while top_y_pos < image_buffer.dimensions().1 {
	let row = get_row_at(&image_buffer, top_y_pos);
	
	if check_line_for_color(&row) {

	    if top_y_pos > LINE_WIDTH {
		top_y_pos -= LINE_WIDTH;
		break;
	    }
	    else {
		top_y_pos = 0;
		break;
	    }
	}
	
	top_y_pos += 1;
    }
    
    // find bottom rectangle line.

    let mut bottom_y_pos = image_buffer.dimensions().1 - 1;
    
    while bottom_y_pos > 0 {
	let row = get_row_at(&image_buffer, bottom_y_pos);	
	
	if check_line_for_color(&row) {
	    if bottom_y_pos + LINE_WIDTH < image_buffer.dimensions().1 {
		bottom_y_pos += LINE_WIDTH;
		break;
	    } else {
		bottom_y_pos = image_buffer.dimensions().1 - 1;
		break;
	    }
	}
	
	bottom_y_pos -= 1;
    }
    
    // find left rectangle line

    let mut left_x_pos = 0;
    
    while left_x_pos < image_buffer.dimensions().0 {
	let col = get_col_at(&image_buffer, left_x_pos);

	if check_line_for_color(&col) {
	    if left_x_pos > LINE_WIDTH {
		left_x_pos -= LINE_WIDTH;
		break;
	    } else {
		left_x_pos = 0;
		break;
	    }
	}
	
	left_x_pos += 1
    }

    // find right rectangle line

    let mut right_x_pos = image_buffer.dimensions().0 - 1;
    
    while right_x_pos > 0 {
	let col = get_col_at(&image_buffer, right_x_pos);	
	
	if check_line_for_color(&col) {
	    if right_x_pos + LINE_WIDTH < image_buffer.dimensions().1 {
		right_x_pos += LINE_WIDTH;
		break;
	    } else {
		right_x_pos = image_buffer.dimensions().1 - 1;
		break;
	    }
	}
	
	right_x_pos -= 1;
    }
    
    // check that rect is valid

    if right_x_pos <= left_x_pos || bottom_y_pos <= top_y_pos {
	return Err("Malformed rectangle")
    }
    
    let rect = Rect::at(left_x_pos as i32, top_y_pos as i32).of_size(right_x_pos - left_x_pos, bottom_y_pos - top_y_pos);
    Ok(DynamicImage::ImageRgba8(draw_hollow_rect(&DynamicImage::ImageRgba8(image_buffer), rect, Rgba([255,0,0,1]))))
    
    // DynamicImage::ImageRgba8(image_buffer).crop(left_x_pos, top_y_pos, right_x_pos - left_x_pos, bottom_y_pos - top_y_pos)
}

pub fn extract_smallest_rect(image: DynamicImage) -> Result<DynamicImage, &'static str> {
    let image_buffer = image.into_rgba8();

    
    const LINE_WIDTH : u32 = 135;
    // find top rectangle line. Should be a straight line that is all white/paper. Find closest to text (so line not all white.) Add some offset. 
    // start in the middle
    let mut top_y_pos = 0;
    
    while top_y_pos < image_buffer.dimensions().1 {
	let row = get_row_at(&image_buffer, top_y_pos);
	
	if check_line_for_color(&row) {

	    if top_y_pos > LINE_WIDTH {
		top_y_pos -= LINE_WIDTH;
		break;
	    }
	    else {
		top_y_pos = 0;
		break;
	    }
	}
	
	top_y_pos += 1;
    }
    
    // find bottom rectangle line.

    let mut bottom_y_pos = image_buffer.dimensions().1 - 1;
    
    while bottom_y_pos > 0 {
	let row = get_row_at(&image_buffer, bottom_y_pos);	
	
	if check_line_for_color(&row) {
	    if bottom_y_pos + LINE_WIDTH < image_buffer.dimensions().1 {
		bottom_y_pos += LINE_WIDTH;
		break;
	    } else {
		bottom_y_pos = image_buffer.dimensions().1 - 1;
		break;
	    }
	}
	
	bottom_y_pos -= 1;
    }
    
    // find left rectangle line

    let mut left_x_pos = 0;
    
    while left_x_pos < image_buffer.dimensions().0 {
	let col = get_col_at(&image_buffer, left_x_pos);

	if check_line_for_color(&col) {
	    if left_x_pos > LINE_WIDTH {
		left_x_pos -= LINE_WIDTH;
		break;
	    } else {
		left_x_pos = 0;
		break;
	    }
	}
	
	left_x_pos += 1
    }

    // find right rectangle line

    let mut right_x_pos = image_buffer.dimensions().0 - 1;
    
    while right_x_pos > 0 {
	let col = get_col_at(&image_buffer, right_x_pos);	
	
	if check_line_for_color(&col) {
	    if right_x_pos + LINE_WIDTH < image_buffer.dimensions().1 {
		right_x_pos += LINE_WIDTH;
		break;
	    } else {
		right_x_pos = image_buffer.dimensions().1 - 1;
		break;
	    }
	}
	
	right_x_pos -= 1;
    }
    
    // check that rect is valid

    if right_x_pos <= left_x_pos || bottom_y_pos <= top_y_pos {
	return Err("Malformed rectangle")
    }
    
    Ok(DynamicImage::ImageRgba8(image_buffer).crop(left_x_pos, top_y_pos, right_x_pos - left_x_pos, bottom_y_pos - top_y_pos))
}


// private functionsx

fn get_row_at(image_buffer : &RgbaImage, y_pos : u32) -> Vec<&Rgba<u8>> {
    let mut row : Vec<&Rgba<u8>> = Vec::new();
    let mut x_pos = 0;
    
    while x_pos < image_buffer.dimensions().0 {
	let pixel = image_buffer.get_pixel(x_pos, y_pos);
	row.push(pixel);
	
	x_pos += 1;
    }

    row
}

fn get_col_at(image_buffer : &RgbaImage, x_pos : u32) -> Vec<&Rgba<u8>> {
    let mut col : Vec<&Rgba<u8>> = Vec::new();
    let mut y_pos = 0;
    
    while y_pos < image_buffer.dimensions().1 {
	let pixel = image_buffer.get_pixel(x_pos, y_pos);
	col.push(pixel);
	
	y_pos += 1;
    }

    col
}

fn check_line_for_color(pixels : &Vec<&Rgba<u8>>) -> bool {
    const COLOR_THRESHOLD : u8 = 10;
    
    for pixel in pixels {
	if pixel.0[0] < COLOR_THRESHOLD && pixel.0[1] < COLOR_THRESHOLD && pixel.0[2] < COLOR_THRESHOLD {
	    return true;
	}
    }

    false
}
