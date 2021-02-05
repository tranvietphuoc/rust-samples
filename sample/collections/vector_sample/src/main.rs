// given a list of integers, use vector and return the mean (average), median (when sorted, the
// value in the middle position) and mode (the value occurs most often; a hash map will by helpful
// here) of the list

use std::io;
use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();
    println!("input the number of element of list");
    let mut string_number = String::new();
    io::stdin().read_line(&mut string_number).expect("Failed to read line");
    let n: u32 = string_number.trim().parse().expect("input the integer");
    println!("Now, input the list elements");

    for _ in 0..n {
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).expect("failed to read line");
        let element: i32 = tmp.trim().parse().expect("please input the number");
        &mut v.push(element);
    }

    let tmp_v = &mut v[..];
    // sort list
    tmp_v.sort();

    // calculate the mean
    let sum_of_vector: i32 = tmp_v.iter().sum() ;
    let mean = (sum_of_vector as f32) / n as f32;
    println!("The mean of vector is: {:.3}", mean);

    // calculate median
    if n % 2 == 0 {
        let index = n / 2 -1;
        println!("the median is: {}", &tmp_v[index as usize]);
    } else {
        let index = n / 2;
        println!("the median is: {}", &tmp_v[index as usize]);
    }

    // calculate mode of list
    // a mode is the value occurs most often
    // here we use HashMap to implement mode of list
    let mut map = HashMap::new();
    for element in tmp_v{
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    println!("the map is: {:?}", map);
    // let max_value = map.values().max();
    // println!("max value: {}", &max_value.unwrap());

}
