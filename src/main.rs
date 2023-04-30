fn main() {
    let a = 2;
    let b = 2;
    let ans = i32::pow(a, 3) + i32::pow(b, 3) + (3 * a * b * (a + b));
    print!("{}", ans);
}
