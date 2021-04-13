# rust_notes

Notes about rust.

## Chapter 1

**Install rust**

On Linux or MacOS

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

**Compile & Rust**

```bash
$ rustc main.rs
$ ./main
Hello, world!
```

### Cargo

Project Manager

**Create project with Cargo**

```bash
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

**Build a Cargo Project**

```bash
$ cargo build
    Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

**Running a Cargo Project**

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

**Check your code**

```bash
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

## Chapter 2

### Crate

Package Manager

**Add package**

Add to Cargo.toml line below:

```toml
[dependencies]
rand = "0.5.5"
```

## Chapter 3

### Variables & Mutability

Variables are immutable by default.

**definition**

```rust
let mut x = 15; // Mutable
let y = 15; // Immutable
const MAX_POINTS: u32 = 100_000;
```

**Constant**
Diffences between immutable value and constant:

- Constants are immutable by type.
- Constants can be define on the global scope.
- Constants can only be defined on a **const** expression, not the result of a function.
- Constants are defined for the entire program runtime.

**Shadowing**

Redefine a variable.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

### Data Types

Rust code uses snake case as the conventional style for function and variable names

#### Scalar Type

**Integer Types**

| Length  | Signed | Unsigned |
| ------- | :----: | -------: |
| 8-bit   |   i8   |       u8 |
| 16-bit  |  i16   |      u16 |
| 32-bit  |  i32   |      u32 |
| 64-bit  |  i64   |      u64 |
| 128-bit |  i128  |     u128 |
| arch    | isize  |    usize |

Example:

| Number literals |   Example   |
| --------------- | :---------: |
| Decimal         |   98_222    |
| Hex             |    0xff     |
| Octal           |    0o77     |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  |    b'A'     |

**Floating-Point Types**
f32, f64

```rust
let x = 2.0; // f64
let y: f32 = 3.0; // f32
// Use underscores to improve readability!
let z = 1_000_000u32; //One million
```

**Numeric Operations**

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

**The Boolean Type**

```rust
let t = true;
let f: bool = false;

println!("true AND false is {}", true && false);
println!("true OR false is {}", true || false);
println!("NOT true is {}", !true);
```

**The Character Type**

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

#### Compound Types

**The Tuple Type**
Variety of types into one compound type.
Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let tup = (500, 6.4, 1);
let (x, y, z) = tup;

let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

**The Array Type**
Have the same type.
An array an't flexible.

```rust

let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];

let a = [3; 5]; // [3, 3, 3, 3, 3]
```

### Functions

A macro ends by "!", a function does not.
An instruction ends by ";".
if a function returns nothing, it returns empty tuple.

```rust

fn function_return_nothing() {
    println!("function_return_nothing");
}

fn function_return_nothing_bis(a: &str) -> (){
    println!("function_return_nothing_bis {}", a);
}

fn incremental_function(a: i64) -> i64{
    let b = a+1;
    return b;
}
fn incremental_function_bis(a: i64) -> i64{a+1}
```

**Function Bodies Contain Statements and Expressions**

```rust

fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}
```

### Control Flow

#### if Expressions

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else {
    println!("number is not divisible by 4, 3, or 2");
}

let number = if condition { 5 } else { 6 }; // OK
let number = if condition { 5 } else { "six" }; // ERROR
```

#### Repetition

```rust
loop {println!("again!");} // INFINITY LOOP
```

**Returning Values from Loops**

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
println!("The result is {}", result);
```

**Conditional Loops with while**

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

**Looping Through a Collection with for**

```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
}

for element in a.iter() {
    println!("the value is: {}", element);
}

for number in (1..4).rev() {// rev : Reverse
    println!("{}!", number);
}
```

## Bibliography

[Rust Book](https://doc.rust-lang.org/book/title-page.html "Rust Book (Start Here)")

[Rust By Example](https://doc.rust-lang.org/rust-by-example/ "Next one")

[Youtube Playlist by Tensor Programming ](https://www.youtube.com/playlist?list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW)
