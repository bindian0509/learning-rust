fn main() {
    let x: i32 = 42;
    let y: i32 = 2;
    let op: i32 = 0;
    let result = calculator(x, y, op);
    println!("{}", result);
    println!("{}", calculator(1, 0, 2));
}

pub fn calculator(x: i32, y: i32, op: i32) -> i32 {
    match op {
        0 => x + y,
        1 => x - y,
        2 => x * y,
        3 => {
            if y == 0 {
                panic!("division by zero");
            } else {
                x / y
            }
        }
        _ => {
            if y == 0 {
                panic!("division by zero");
            } else {
                x % y
            }
        },
    }
}
