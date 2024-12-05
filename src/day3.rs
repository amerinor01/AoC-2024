use std::fs;
use regex::Regex;
use find_all::FindAll;

fn get_until_x(input: &str) -> &str {
    if let Some(pos) = input.find(')') {
        &input[..pos]
    } else {
        input  // Return the entire string if 'x' is not found
    }
}
pub fn first() {
    let file_path = String::from("inputs/day3.txt");

    let re = Regex::new(r"^\d+,\d+$").unwrap();  // Create regex to match strings that start and end with a digit

    let r = fs::read_to_string(file_path.clone())
        .expect("Should have been able to read the file");
    let a: i32 = r.split("mul(")
        .map(|s| get_until_x(s))
        .filter(|s| re.is_match(s))
        .map(|s| {
            let first: i32 = s.split(',').collect::<Vec<_>>()[0].parse().unwrap();
            let second: i32 = s.split(',').collect::<Vec<_>>()[1].parse().unwrap();
            first * second
        })
        .sum();

    println!("{:?}", a);
    
}

pub fn second() {
    let file_path = String::from("inputs/day3.txt");
// Create a regex to match 'do()'
    let rdo = Regex::new(r"do\(\)").unwrap();
    let rdont = Regex::new(r"don\'t\(\)").unwrap();

    // Use find_iter to find all matches of 'do()'
    //let re_coma = Regex::new(r"^\d+,\d+$").unwrap();  // Create regex to match strings that start and end with a digit
    //

    //free_time = 

    let r = fs::read_to_string(file_path.clone())
        .expect("Should have been able to read the file");

    let p1: Vec<_> = rdo.find_iter(&r)
        .map(|m| m.end())
        .collect(); // Collect the matches into a vector
    println!("{:?}", p1);
    let p2: Vec<_> = rdont.find_iter(&r)
        .map(|m| m.end())
        .collect(); // Collect the matches into a vector
    println!("{:?}", p2);
    println!("{:?}", p3);


    //Get the ranges between values
    //To compute the i need to check taht form 0 til first don't, then from the end of this don't
    //to the closet do, then next dont
    //
    



    let a: i32 = r.split("mul(")
        //.step_by(2)
        .map(|s| {
            println!("{:?}", s);  0
        })
        //.map(|s| get_until_x(s))
        //.filter(|s| re-coma.is_match(s))
        //.map(|s| {
        //    0
        //})
        .sum();

    println!("{:?}", a);
}
