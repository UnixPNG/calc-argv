fn tipe<T>(_: &T) -> &str {
    let a = std::any::type_name::<T>();
    a
}

fn math(inp: String) -> i32 {
    if inp.contains('+') {
        let x: Vec<&str> = inp.split("+").collect();
        let a = x[0].parse::<i32>().unwrap();
        let b = x[1].parse::<i32>().unwrap();
        a + b
    } else if inp.contains('/'){
        let x: Vec<&str> = inp.split("/").collect();
        let a = x[0].parse::<i32>().unwrap();
        let b = x[1].parse::<i32>().unwrap();
        a / b
    } else if inp.contains("*"){
        let x: Vec<&str> = inp.split("*").collect();
        let a = x[0].parse::<i32>().unwrap();
        let b = x[1].parse::<i32>().unwrap();
        a * b
    } else if inp.contains("-"){
         let x: Vec<&str> = inp.split("-").collect();
        let a = x[0].parse::<i32>().unwrap();
        let b = x[1].parse::<i32>().unwrap();
        a - b
    } else {
         0
    }
}
fn main() {
    println!("math problem (+-/*):");
    let mut inp: String = "".to_owned();
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);
    for i in args {
        inp.push_str(&i);
    }
    println!("{}",inp);
    let a = math(inp);
    println!("{}",a);
}
