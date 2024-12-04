use std::collections::HashMap;
use std::fs;

pub fn first() {
    // --snip--
    let file_path = String::from("inputs/day1.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let v = contents
        .split('\n')
        .flat_map(|str| str.split("   "))
        .map(str::to_owned);
    let mut merged = v.collect::<Vec<_>>();
    merged.pop();

    let mut odd: Vec<i32> = merged
        .iter()
        .skip(1)
        .step_by(2)
        .map(|value| value.parse().unwrap())
        .collect();
    odd.sort();
    let mut even: Vec<i32> = merged
        .iter()
        .step_by(2)
        .map(|value| value.parse().unwrap())
        .collect();
    even.sort();

    let result: i32 = odd
        .iter()
        .zip(even.iter())
        .map(|(a, b)| i32::abs(a - b))
        .sum();

    println!("{:?}", result);
}

pub fn second() {
    // --snip--
    let file_path = String::from("inputs/day1.txt");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut merged: Vec<_> = contents
        .split('\n')
        .flat_map(|str| str.split("   "))
        .map(str::to_owned)
        .collect();
    //let mut merged = v.collect::<Vec<_>>();
    merged.pop();

    let mut odd: Vec<i32> = merged
        .iter()
        .skip(1)
        .step_by(2)
        .map(|value| value.parse().unwrap())
        .collect();
    odd.sort();
    let mut even: Vec<i32> = merged
        .iter()
        .step_by(2)
        .map(|value| value.parse().unwrap())
        .collect();
    even.sort();

    let appears: HashMap<_, _> = odd.iter().fold(HashMap::new(), |mut acc, &item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });

    let result: i32 = even
        .iter()
        .map(|value| appears.get(value).unwrap_or(&0) * value)
        .sum();

    println!("{:?}", result);
}
