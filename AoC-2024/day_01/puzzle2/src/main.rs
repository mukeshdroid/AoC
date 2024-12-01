use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_count_hashmap(nums: Vec<u32>) -> HashMap<u32, u32> {
    let mut freq: HashMap<u32, u32> = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }
    freq
}

fn main() {
    //open the file

    let input_file = match File::open("../input.txt") {
        Ok(input) => {
            println!("Opened Sucessfully");
            input
        }
        Err(_) => {
            println!("No such file or directory!");
            return;
        }
    };

    //initialize the read buffer
    let reader = BufReader::new(input_file);

    //initialize two empty Vectors
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    //iterate over the file line by line and append the numbers to respective lists
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                println!("Error reading line");
                return;
            }
        };

        let num: Vec<&str> = line.split("   ").collect();

        list1.push(num.get(0).unwrap().parse().unwrap());
        list2.push(num.get(1).unwrap().parse().unwrap());
    }

    // convert to dictionary with with numbers as key and their count
    let list1_freq = get_count_hashmap(list1);
    let list2_freq = get_count_hashmap(list2);

    //sum up the corresponding distances
    let mut similarity: u32 = 0;
    for i in list1_freq.keys() {
        similarity = similarity + i * list2_freq.get(i).copied().unwrap_or_default();
    }
    println!("{}", similarity);
}
