use std::io;
use std::collections::HashMap;

fn s1() {
    // read the file until EOF
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    loop {
        let mut input = String::new();
        input = match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => break,
        };
        if input.trim().is_empty() {
            break;
        }   
        // split the input into two int
        let mut iter = input.split_whitespace();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let b = iter.next().unwrap().parse::<i32>().unwrap();
        list_a.push(a);
        list_b.push(b);
    }

    // sort both lists
    list_a.sort();
    list_b.sort();

    let mut result = 0;

    for i in 0..list_a.len() {
        let a = list_a[i];
        let b = list_b[i];
        result += (a - b).abs();
    }

    println!("{}", result);
}


fn s2() {
    // read the file until EOF
    // let mut list_a = Vec::new();
    let mut map_a = HashMap::new();
    let mut list_b = Vec::new();
    loop {
        let mut input = String::new();
        input = match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => break,
        };
        if input.trim().is_empty() {
            break;
        }   
        // split the input into two int
        let mut iter = input.split_whitespace();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let b = iter.next().unwrap().parse::<i32>().unwrap();
        list_b.push(b);
        if map_a.contains_key(&a) {
            let count = map_a.get(&a).unwrap() + 1;
            map_a.insert(a, count);
        } else {
            map_a.insert(a, 1);
        }
    }

    let mut result = 0;

    for i in 0..list_b.len() {
        let b = list_b[i];
        // if b in list_a, sum it
        if map_a.contains_key(&b) {
            let count = map_a.get(&b).unwrap();
            result += count * b;
        }
    }

    println!("{}", result);
}

fn main() {
    s1();
    s2();
}