use std::mem;

fn main() {
    let t = (1, 'a', false);
    let f = (2, (1, 'a', false));
    println!("{} {} {}", t.0, t.1, t.2);
    println!("{:?}", t);
    println!("{:?}", f); //DEBUG Flag
    println!("{:#?}", f); //Pretty DEBUG Flag

    let xs: [i32; 5] = [4, 5, 6, 7, 8];
    let ys = &xs[2..5];
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));
    println!("{} {} {}", ys[0], ys.len(), mem::size_of_val(&ys));
    println!("{:?}", xs);
    println!("{:?}", ys);

    let s: &str = "String";
    let s: String = "String".to_string();
    let ss: String = String::from("String!");

    let slice = &ss[1..4];
    println!("{}", slice);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let s = h + &w;
    println!("{}", s);

    let t = ();
}
