use std::collections::HashMap;
use std::fs;

pub fn first() {
    // --snip--
    let file_path = String::from("inputs/day2.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut l: Vec<_> = contents.split('\n').collect();
    l.pop();

    let mut foo: i32 = 0;
    let r: i32 = l
        .iter()
        .map(|s| {
            let inner: Vec<i32> = s
                .split_whitespace()
                .map(|v| v.parse().unwrap()) // Parse each part as i32
                .collect();

            let (min, max) = inner
                .windows(2)
                .map(|v| v[0] - v[1])
                .fold((i32::MAX, i32::MIN), |(min, max), x| {
                    (min.min(x), max.max(x))
                });
            if (min > 0 && max > 0) || (min < 0 && max < 0) {
                if min >= -3 && max < 4 {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    println!("{:?}", r);
    /*
        for i in l {
            let result = i.split(' ')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
                .map(|v|  v[0] - v[1])
                .fold(
                    (i32::MAX, i32::MIN),
                    |(min, max), x| {
                        (min.min(x), max.max(x))
                    },
                );

            let x = result.0;
            let y = result.1;
            if(x > 0 && y > 0) || (x < 0 && y < 0){  // both positive or both negative
                if(x >= -3 && y < 4){
                    foo = foo + 1;
                    println!("{:?}", result);
                }

            }

        }

        println!("{:?}", foo);

        let result: Vec<i32> = l
            .iter()                             //For each list
            .map(|v| v.split(' '))              //split by the ' ' character
            .map(|value| value.parse().unwrap())
            .collect::<Vec<i32>>();
            //.map(str::to_owned)
            .map(|v| v.parse().unwrap())

            .collect::<Vec<_>>();
        println!("{:?}", result);
    */
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
