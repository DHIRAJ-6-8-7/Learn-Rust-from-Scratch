fn main() {
    let mut i = 0;
    let mut counter = 0;
    loop {
        println!("hello world {i}");
        i = i + 1;
        if i == 990 {
            break;
        }
    }
    let check = loop {
        counter += 1;
        if counter == 10000 {
            break "happy";
        }
    };
    println!("{check}");

    //loop label
    'outer_loop: loop {
        println!("outer loop {counter}");
        counter += 1;
        let mut x = 0;
        'inner_loop: loop {
            println!("inner loop {x}");
            if counter % 17 == 1 {
                break 'outer_loop;
            }
            if x % 11 == 0 {
                break 'inner_loop;
            }
            x += 1;
        }
    }
    let mut i = 0;
    while i != 99 {
        println!("{i}");
        i += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }

    for i in 1..100 {
        println!("{i}");
    }
    for i in (1..100).rev() {
        println!("{i}");
    }
}
