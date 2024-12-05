use std::fs;

pub fn first() {
    let file_path = String::from("inputs/day2.txt");

    let r: i32 = fs::read_to_string(file_path.clone())
        .expect("Should have been able to read the file")
        .lines()
        .filter(|s| { //use of filtermap to avoid empty lines
            let inner: Vec<i32> = s
                .split_whitespace()
                .filter_map(|v| v.parse().ok()) // parse each part as i32
                .collect();

            if inner.len() < 2 {
                return false; //Skip small arrays with less that 2 elements
            }

            let (min_diff, max_diff) = inner
                .windows(2)
                .map(|v| v[0] - v[1])
                .fold((i32::MAX, i32::MIN), |(min_diff, max_diff), x| {
                    (min_diff.min(x), max_diff.max(x))
                });

            // Correct the logical expression
            let valid_sign = (min_diff > 0 && max_diff > 0) || (min_diff < 0 && max_diff < 0); // Both are either positive or negative
            let valid_range = min_diff >= -3 && max_diff < 4; // Range check
            valid_sign && valid_range // Combine both conditions    
        })
        .count() as i32;
    println!("{:?}", r);
}

pub fn second() {
    let file_path = String::from("inputs/day2.txt");

    let r: i32 = fs::read_to_string(file_path.clone())
        .expect("Should have been able to read the file")
        .lines()
        .filter(|s| { //use of filtermap to avoid empty lines
            let inner: Vec<i32> = s
                .split_whitespace()
                .filter_map(|v| v.parse().ok()) // parse each part as i32
                .collect();

            if inner.len() < 2 {
                return false; //Skip small arrays with less that 2 elements
            }


            let (min_diff, max_diff) = inner
                .windows(2)
                .map(|v| v[0] - v[1])
                .fold((i32::MAX, i32::MIN), |(min_diff, max_diff), x| {
                    (min_diff.min(x), max_diff.max(x))
                });

            // Correct the logical expression
            let valid_sign = (min_diff > 0 && max_diff > 0) || (min_diff < 0 && max_diff < 0); // Both are either positive or negative
            let valid_range = min_diff >= -3 && max_diff < 4; // Range check
            if !(valid_sign && valid_range){ // Combine both conditions    
            let diffs: Vec<_> = inner
                .windows(2)
                .filter_map(|v| 
                    if ((v[0] - v[1]).abs() > 3) || ((v[0] - v[1]).abs() == 0 ){
                        Some(v[0] - v[1])
                    }else{
                            None
                        }
                )
                     
                .collect();
                println!("{:?}", diffs);
            }else {
                return true;
            }
            false
/*
            let mut sign = false;
            let mut first_assigned = false;
            let mut errors = 0;

            for i in diffs:
                if !first_assigned:
                    sign = i > 0;
                    first_assigned = true;

                //Dereasing
                if(sign){
                    if(i < 1 ){
                        errors = errors + 1;
                    }
                }else {
            }
 */                   

        })
        .count() as i32;
    println!("{:?}", r);
}
