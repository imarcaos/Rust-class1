//for the example4
use std::fs::File;
fn main() {
    //example1
    //panic!("Farewell");

    //example2
    //let v = vec![0,1,2,3];
    //println!("{}", v[6]);

    //example3
    /*
    let fruits = vec!["banana", "apple", "coconut"];

    let first = fruits.get(0);
    println!("{:?}", first);

    let third = fruits.get(2);
    println!("{:?}", third);

    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
    */

    //example4
    /*
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Can't open the file: {:?}", error),
    };
    */

    //exemple5, do the same at the example4
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}
