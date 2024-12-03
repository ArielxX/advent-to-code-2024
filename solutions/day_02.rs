use std::io;

fn check_list(list: &Vec<i32>, monotony: i32, start: usize) -> bool {
    for i in start..list.len()-1 {
        let a = list[i];
        let b = list[i+1];
        
        if (a - b).abs() > 3 || monotony != (b - a).signum() {
            return false;
        }
    }
    
    return true;
}

fn s1() {
    let mut result = 0;

    // read the file until EOF
    loop {
        let mut input = String::new();
        input = match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => break,
        };
        if input.trim().is_empty() {
            break;
        }   
        // split the input into several ints
        let mut report = input.split_whitespace();

        // build a list from input
        let mut list = Vec::new();
        for r in report {
            list.push(r.parse::<i32>().unwrap());
        }

        if check_list(&list, 1, 0) || check_list(&list, -1, 0) {
            result += 1;
        }
    }

    println!("{}", result);
}


fn check_list_safety(list: &Vec<i32>, monotony: i32) -> bool {
    let mut b = list[0];
    let mut safety = false;

    for i in 0..list.len()-1 {
        let a = b;
        b = list[i+1];
        
        if (a - b).abs() > 3 || monotony != (b - a).signum() {
            if safety {
                return false
            }

            if i > 0 && (list[i-1] - b).abs() <= 3 && monotony == (b - list[i-1]).signum() && check_list(&list, monotony, i + 1) {
                return true;
            }
            if i == 0 && check_list(&list, monotony, 1) {
                return true;
            }

            safety = true;
            b = a;
        }
    }

    return true;
}

fn s2() {
    let mut result = 0;

    // read the file until EOF
    loop {
        let mut input = String::new();
        input = match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => break,
        };
        if input.trim().is_empty() {
            break;
        }   
        // split the input into several ints
        let mut report = input.split_whitespace();

        // build a list from input
        let mut list = Vec::new();
        for r in report {
            list.push(r.parse::<i32>().unwrap());
        }

        if check_list_safety(&list, 1) || check_list_safety(&list, -1) {
            result += 1;
        }
    }

    println!("{}", result);
}

fn main() {
    // s1();
    s2();   
}