use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    //open the file

    let input_file = match fs::read_to_string("../input.txt") {
        Ok(contents) => {
            println!("Opened Sucessfully");
            input
        }
        Err(_) => {
            println!("No such file or directory!");
            return;
        }
    };

//------------------------TO DO -------------------------------
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

    //sort the vectors
    list1.sort();
    list2.sort();

    //sum up the corresponding distances
    let mut sum: u32 = 0;
    for i in 0..list1.len() {
        sum = sum + list1[i].abs_diff(list2[i]);
    }

    println!("{}", sum);
}