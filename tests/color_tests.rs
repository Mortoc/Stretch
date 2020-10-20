use stretch::renderer::color::Color;

#[macro_use]
extern crate approx;

#[test]
pub fn color_constructor_verification() {
    let color = Color::new(0.1, 0.2, 0.3, 0.4);
    assert_relative_eq!(color.r(), 0.1);
    assert_relative_eq!(color.g(), 0.2);
    assert_relative_eq!(color.b(), 0.3);
    assert_relative_eq!(color.a(), 0.4);
}
