fn main() {
    //i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    //f32, f64

    let a = 1 + 20;
    let s = 51 - 44;
    let d = 4 / 6;
    let d = 49 % 6;

    //bool: true/false
    let c: char = 'z';
    println!("{}", c);

    let t: (i32, f64, char) = (42, 6.12, 'j');

    let (_, _, x) = t;
    t.0;

    let a = [1, 1, 2, 4, 5];
    let a: [i32; 5] = [1, 1, 2, 4, 5];
    let a1 = a[0];
}
