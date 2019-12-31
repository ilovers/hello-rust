fn main() {
    test_ownership();
    //test_string();
    //test_for();
    //test_while();
    //test_loop();
    //test_if();
    //test_expression();
    //another_function(x);
    //test_array();
    //test_shadow();
    //test_variables();
}

fn test_ownership() {
    // function with move value
    let s = String::from("hello");
    func_move(s);
    println!("{}", s);

    /*
    // copy
    let a = 5;
    let b = a;
    println!("{}, {}", a, b);
    */

    /*
    // error: borrow of moved value: `s1`
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, {}", s1, s2);
    */

    /*
    // clone `deep copy`
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
    */
}

fn func_move(s: String) {
    println!("{}", s);
}

/*
fn test_string() {
    let mut s = "Hello, ";
    println!("{}", s);
    s = "123";
    println!("{}", s);

    let mut s = String::from("Hello, ");
    s.push_str("world!");
    println!("{}", s);
}

fn test_for() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}

fn test_while() {
    let mut i = 3;
    while i != 0 {
        println!("{}", i);
        i -= 1;
    }
}

fn test_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

fn test_if() {
    let condition = true;
    let x = if condition {
        5
    } else {
        6
    };
    println!("{}", x)
}

fn five() -> u32 {
    5
}

fn test_expression() {
    let x = 3;
    let y = {
        let x = 4;
        x + 1
    };
    println!("{}", y)
}

fn another_function(x: u32) {
    println!("{}", x)
}

fn test_array() {
    let a: [i32; 3] = [1, 2, 3];
    let index = 10;
    let v = a.get(index);
    println!("{}", Some(v));
}

fn test_shadow() {
    let space = "   ";
    println!("{}", space);
    let space = space.len();
    println!("{}", space);
}

fn test_variables() {
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);
}
*/
