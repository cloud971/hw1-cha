use std::io::Write;
use std::str::FromStr;


// from rust programming book
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
    let check: Vec<String> = std::env::args().collect();

    if check.len() == 3 {
        println!("{}", check[2]);
        return;
    }

    if check.len() == 2 {
        println!(0);
        return;
    }

    for x in 2..check.len() {
        numbers.push(u64::from_str(&check[x]).expect("error parsing argument"));
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

    if check[1] == "product" {
        let mut prod = 1;
        for x in &numbers[0..] {
            prod = prod * x;
        }

        println!("{}", prod);
    }

    if check[1] == "sum" {
        let mut sum = 0;
        for x in &numbers[0..] {
            sum += x;
        }
        println!("{}", sum);
    }

    if check[1] == "lcm" {
        let mut answer = numbers[0];
        for x in &numbers[1..] {
            answer = (answer * x) / gcd(*x, answer);
        }

        println!("{}", answer);
    }
}

#[test]

fn test_sum(){

 let v = vec![1,2,3,4];

    let mut sum = 0;
    let mut product = 1;

    for x in &v{
        sum += x;
        product *= x;
    }
    assert_eq!(sum,10);
    assert_eq!(product,24);


}

#[test]
fn test_product(){

    let v = vec![1,2,3,4];
    let mut product = 1;
    for x in &v{
        product *= x;
    }
    assert_eq!(product,24);
}

#[test]

fn test_gcd(){

    let v = vec![20,50,150,120];
    let mut d = v[0];
    for m in &v {
        d = gcd(d, *m);
    }
    assert_eq!(d,10);
}

#[test]
fn test_lcm(){

    let v = vec![20, 50, 4];
    let mut answer = v[0];
    for x in &v[1..] {
        answer = (answer * x) / gcd(*x, answer);
    }

    assert_eq!(100,answer);
}

