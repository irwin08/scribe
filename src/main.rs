use image::io::Reader as ImageReader;

mod image_processor;

fn main() {
    println!("Hello, world!");
    let img = ImageReader::open("myimage.jpg").unwrap().decode().unwrap();

    //let img2 = image_processor::extract_smallest_rect(img);
    let img2 = image_processor::draw_smallest_rect(img);
    
    img2.unwrap().save("test.png").unwrap();
}
