use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut num_list: Vec<u32> = Vec::new();


    // Fill vector list of numbers from input
    for line in f.lines() {
        let curr_num:u32 = line.unwrap().trim().parse().unwrap();
        num_list.push(curr_num);
    }

    // Question 1

    // Loop thru each number
    for i in (0..num_list.len()).rev() {
        // Loop thru each remaining number
        for j in (i..num_list.len()).rev() {
            // If they add to 2020, we found it
            if num_list[i] + num_list[j] == 2020 {
                // Could break here for efficiency
                println!("{}, {}", num_list[i], num_list[j]);
                // Display product!
                println!("Product: {}", num_list[i] * num_list[j]);
            }
        }
    }

    // Question 2

    // Loop thru each number
    for i in (0..num_list.len()).rev() {
        // Loop thru each remaining number
        for j in (i..num_list.len()).rev() {
            // Loop thru again
            for k in (j..num_list.len()).rev() {
                // Same as above, if all three add to 2020 we have solution
                if num_list[i] + num_list[j] + num_list[k] == 2020 {
                    // Again, should break here for efficiency
                    println!("{}, {}, {}", num_list[i], num_list[j], num_list[k]);
                    // Display product
                    println!("Product: {}", num_list[i] * num_list[j] * num_list[k]);
                }
            }
            
        }
    }



    Ok(())
}
