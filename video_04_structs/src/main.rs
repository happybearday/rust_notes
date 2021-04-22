use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// Method
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}
// Related function
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object {
            width: width,
            height: height,
        }
    }
}

impl fmt::Display for Object{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) and area : {}", self.width, self.height, self.area())
    }
}

fn test_01() {
    let o = Object {
        width: 35,
        height: 55,
    };

    println!("{}", o);
    println!("{:?}", o);
    println!("{:#?}", o);
}

fn main() {
    test_01();
}
