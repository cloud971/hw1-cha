use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {

    let mut numbers = Vec::new();
    let check:Vec<String> = std::env::args().collect();


    if check.len() == 4{
        println!("{}",check[3]);
        return;
    }

    for x in 3..check.len() {

        numbers.push(u64::from_str(&check[x])
            .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "error ...").unwrap();
        std::process::exit(1);
    }

    if check[1] == "gcd" {
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);
        }
        println!("{}", d);
    }

    if check[2] == "product"{
        let mut prod = 1;
        for x in &numbers[0..]{
            prod = prod*x;
        }

        println!("{}",prod);
    }

    if check[2] == "sum"{
        let mut sum = 0;
        for x in &numbers[0..]{
            sum+=x;
        }
        println!("{}",sum);
    }
/*
    if check[2] == "lcm"{

    }
    */
}