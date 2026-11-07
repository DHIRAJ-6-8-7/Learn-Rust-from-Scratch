#![allow(unused)]
struct ThreeDSpacePoint(i64, i64, i64, String);
struct ColorOfPixel(u32, u32, u32, bool);
fn main() {
    let xyz: ThreeDSpacePoint = ThreeDSpacePoint(23, 13, -12, String::from("right hand system"));

    println!("{},{},{}: {}", xyz.0, xyz.1, xyz.2, xyz.3);

    let oled: ColorOfPixel = ColorOfPixel(255, 210, 0, false);
    println!("{},{},{}: {}", oled.0, oled.1, oled.2, oled.3);

    //to destructure the tuple struct
    let ThreeDSpacePoint(x, y, z, orientation) = xyz;
}
