fn test_01() {
    let s = String::from("String test");
    // After this, s will not exist
    let y = s;

    println!("{}", y);
}

fn test_02() {
    let s = String::from("String test");

    // y is a reference of std::string::String
    let y: &String = &s;

    println!("{}", s);
    println!("{}", y);
    println!("{}", s);
}

fn take(v: Vec<i32>) {
    println!("We took v: {}", v[10] + v[100]);
}

fn test_03() {
    let mut v = Vec::new(); //Heap

    for i in 1..1000 {
        v.push(i);
    }

    // Transfert the ownership from test_03 to take
    // The ownership is never return
    take(v);
    println!("Finished!");
}

fn cop(a: i32, b: i32) {
    println!("{}", a + b);
}

fn test_04() {
    let a = 32;
    let b = 45;
    cop(a, b);
    println!("we have a: {} and b: {}", a, b);
}

fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[11]);
}

fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[11]);
}

fn test_05() {
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }
    v = re(v);
    borrow1(&v);
    println!("Stil own v: {} {}", v[0], v[1]);
    borrow2(&v);
    println!("Stil own v: {} {}", v[0], v[1]);
}

fn change_value(s: &mut i8) {
    // Dereferencing 's' and changing its value:
    *s = 5;
}

fn test_06() {
    let mut x = 10;
    println!("x = {}", x);
    // Pass 'x' as a mutable reference:
    change_value(&mut x);
    // Print new value of 'x':
    println!("New value of x = {}", x);

    let y = &mut x;
    *y += 1;
    println!("New value of x = {}", x);
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}

fn test_07() {
    let v = vec![4, 5, 5, 4, 1, 5, 7, 1, 2, 3, 1, 1, 7, 7];
    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }
    println!("{}", v[1]);
}

fn main() {
    test_07()
}
