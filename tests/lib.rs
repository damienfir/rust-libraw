extern crate libraw;


// #[test]
fn load_write() {
    let im = libraw::Image::new("resources/image.dng");
    assert!(im.load());
    assert!(im.unpack());
    assert!(im.to_image());
    assert!(im.write("resources/out.tiff"));
}

#[test]
fn load_copy() {
    let im = libraw::Image::new("resources/image.dng");
    assert!(im.load());
    assert!(im.unpack());
    assert!(im.to_image());
    let (data, h, w, c) = im.copy();
}
