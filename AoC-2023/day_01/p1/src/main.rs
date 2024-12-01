use std::fs::File;
use std::io::{self, BufRead};

fn extract_num(s : String) -> u32{
    let mut nums: Vec<u32> = Vec::new();
    for c in s.chars(){
        if c.is_numeric(){
            nums.push(c.to_digit(10).unwrap());
        }
    }
    nums.get(0).unwrap() * 10 + nums.last().unwrap()
}

fn main() {
    //the path to the input file
    let file_path = "res/input.txt";

    //open the inout file
    let input_file = File::open(file_path).unwrap();

    //create a reader to get lines
    let reader = io::BufReader::new(input_file);
    
    let mut sum: u32 = 0;

    //Iterate over the lines
    for line in reader.lines(){
        let line = line.unwrap();

        sum += extract_num(line); 
    }
    
    println!("{}",sum);
}
