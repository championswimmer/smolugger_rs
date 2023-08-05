use rand::Rng;

fn main() {
    let a = 10;
    let b = 20;

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);

    let c = fun1(a) + fun2(b);
    println!("The value of c is: {}", c);
}

fn fun1(a: u32) -> u32 {
    // generate a random integer below 50
    let rand = rand::thread_rng().gen_range(0..100);
    a * rand
}

fn fun2(a: u32) -> u32 {
    // generate a random integer below 50
    let rand = rand::thread_rng().gen_range(0..100);
    a * rand
}