use regex::Regex;
use std::io;

fn s1() {
    // read line from stdin
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    let mut result: i32 = 0;
    loop {
        let mut input: String = String::new();
        input = match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => break,
        };
        if input.trim().is_empty() {
            break;
        }  
        // detect every mul(x,y) where x and y are integers with 1-3 digits
        // iterate over the regex matches
        for cap in re.captures_iter(&input) {
            let x: i32 = cap[1].parse::<i32>().unwrap();
            let y: i32 = cap[2].parse::<i32>().unwrap();
            println!("{}x{}", x, y);
            result += x * y;
        }
        println!("{}", result);
    }

    println!("{}", result);
}

fn s2() {
    // read line from stdin
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do: Regex = Regex::new(r"do\(\)").unwrap();
    let re_dont: Regex = Regex::new(r"don't\(\)").unwrap();
    
    let mut result: i32 = 0;
    let mut do_mul: bool = true;  

    let mut full_input: String = String::new();

    loop {
        let mut input: String = String::new();
        input = match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => break,
        };
        if input.trim().is_empty() {
            break;
        }  
        full_input.push_str(&input);
    }

    // find the positions of the do and dont
    let mut do_poss: Vec<usize> = Vec::new();
    let mut dont_poss: Vec<usize> = Vec::new();
    print!("do positions: ");
    for cap in re_do.find_iter(&full_input) {
        do_poss.push(cap.start());
        print!("{}, ", cap.start());
    }
    println!();
    print!("dont positions: ");
    for cap in re_dont.find_iter(&full_input) {
        dont_poss.push(cap.start());
        print!("{}, ", cap.start());
    }
    println!();
    
    let mut do_pos: i32 = -1;
    let mut dont_pos: i32 = -1;
    
    // detect every mul(x,y) where x and y are integers with 1-3 digits
    // iterate over the regex matches
    for iter in re.find_iter(&full_input) {
        let pos = iter.start();
        
        // get capture 

        while do_pos + 1 < do_poss.len() as i32 && do_poss[(do_pos + 1) as usize] < pos {
            do_pos += 1;
        }
        while dont_pos + 1 < dont_poss.len() as i32 && dont_poss[(dont_pos + 1) as usize] < pos {
            dont_pos += 1;
        }

        do_mul = dont_pos == -1 || (do_pos != -1 && do_poss[do_pos as usize] > dont_poss[dont_pos as usize]);

        if do_mul{
            // get capture from match
            let cur_cap = re.captures(&full_input[pos..]).unwrap();

            let x: i32 = cur_cap[1].parse::<i32>().unwrap();
            let y: i32 = cur_cap[2].parse::<i32>().unwrap();
            println!("{}x{}", x, y);
            result += x * y;
        }
    }

    println!("{}", result);
}

fn main() {
    // s1();
    s2();   
}