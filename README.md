# rust_math
Math library written in Rust.
Operations for integer lists.
## Adding to your project
Just add ```rust_math = { git = "https://github.com/bktech2021/rust_math" }``` to Cargo.toml file.
## Functions For lists
### rust_math::list::bubble_sort(list: &mut [usize]) { ... } :
*Sorts the list with algorithm bubble sort*
### rust_math::list::med(list: &[usize]) -> i32 { ... } :
*This function returns median of the list*
### rust_math::list::safe_med(list: &[usize]) -> i32 { ... } :
*If list is sorted returns median of list, else -1.*
### rust_math::list::avg(list: &[usize]) -> f32 { ... } :
*This function returns average of the list*
### rust_math::list::mode(list: &[usize]) -> i32 { ... } :
*This funtion returns mode of list.*
### rust_math::list::std_devitation(list: &[usize]) -> f32 { ... } :
*This function returns the standard deviation of the list.*
### rust_math::list::highest(list: &[usize]) -> i32 { ... } :
*This function returns the highest number in the list.*
### rust_math::list::lowest(list: &[usize]) -> i32 { ... } :
*This function returns the lowest number in the list.*
### rust_math::list::check_sorted(list: &[usize]) -> bool { ... } :
*Returns true if list is sorted, else false.*
## For numbers
### rust_math::num::factorial(num: i128) -> i128 { ... } :
*Returns factorial of number. (i128 is due to overflow)*
### rust_math::num::is_prime(number: i32) -> bool { ... } :
*Returns true if prime. Else false.*
### rust_math::num::gcd(a: i32, b: i32) -> i32 { ... } :
*Finds greatest common divisor of two numbers*
### rust_math::num::lcm(a: i32, b: i32) -> i32 { ... } :
*Finds least common of 2 numbers*
## Trigonometry
### rust_math::trigonometry::sin(ang: f32) -> f32 { ... } :
*Finds sine of angle in degrees*
### rust_math::trigonometry::cos(ang: f32) -> f32 { ... }:
*Finds cosine of angle in degrees*
### rust_math::trigonometry::tan(ang: f32) -> f32 { ... } :
*Finds tangent of angle in degrees*
### rust_math::trigonometry::cotan(ang: f32) -> f32 { ... } :
*Finds cotangent of angle in degrees*
### rust_math::trigonometry::sec(ang: f32) -> f32 { ... } :
*Finds secant of angle in degrees*
### rust_math::trigonometry::cosec(ang: f32) -> f32 { ... } :
*Finds cosecant of angle in degrees*
### rust_math::trigonometry::deg2rad(ang: f32) -> f32 { ... } :
*Converts degrees to radians*
### rust_math::trigonometry::rad2deg(ang: f32) -> f32 { ... } :
*Converts radians to degrees*
### rust_math::trigonometry::find_quadrant(angle: f32) -> i32 { ... } :
*Finds quadrant of angle in degrees*