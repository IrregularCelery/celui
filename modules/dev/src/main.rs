use celui::{Color, Mat2, Mat3, Mat4, Rect, Vec2, Vec3, Vec4};

fn main() {
    println!("{:?}", Vec2::new(1.0, 2.0));
    println!("{:?}", Vec3::new(1.0, 2.0, 3.0));
    println!("{:?}", Vec4::new(1.0, 2.0, 3.0, 4.0));

    println!("{:?}", Mat2::identity());
    println!("{:?}", Mat3::identity());
    println!("{:?}", Mat4::identity());

    println!("{:?}", Rect::new(0.0, 0.0, 32.0, 32.0));

    println!("{:?}", Color::BLACK);
}
