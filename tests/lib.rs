extern crate libraw;


#[test]
fn load_write() {
    let im = libraw::Image::new("resources/image.dng");
    assert!(im.load());
    assert!(im.write("resources/out.tiff"));
}
