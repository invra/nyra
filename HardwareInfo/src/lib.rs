use rand::prelude::*;
use std::ffi::{c_char, CStr};

#[repr(C)]
#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn add(left: i64, right: i64) -> i128 {
    (left + right).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn say_hello(name: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(name) };
    println!("Hello, {}", c_str.to_str().unwrap())
}

#[unsafe(no_mangle)]
pub extern "C" fn random_point() -> Point {
    let mut rng = rand::thread_rng();
    Point {
        x: rng.r#gen::<u32>(),
        y: rng.r#gen::<u32>(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn distance(first: &Point, second: &Point) -> f64 {
    let dx: f64 = (second.x - first.x).into();
    let dy: f64 = (second.y - first.y).into();

    println!("calculating distance...");

    (dx.powf(2.0) + dy.powf(2.0)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn say_hello_test() {
        let _result = say_hello("Khalid".as_ptr() as *const i8);

        assert!(true)
    }

    #[test]
    fn get_random_point() {
        let point: Point = random_point();
        println!("{:?}", point);
        assert!(true);
    }

    #[test]
    fn can_calculate_distance() {
        let one = Point { x: 1, y: 1 };
        let two = Point { x: 2, y: 2 };

        let result = distance(&one, &two);
        println!("distance between {:?} and {:?} is {}", one, two, result);
        assert!((result - 1.414).abs() < 0.001)
    }
}
