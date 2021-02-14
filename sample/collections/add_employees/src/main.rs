use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;

fn create_employee(map: &mut HashMap<String, Vec<String>>) {
    // Problem:
    // Using a hash map and vectors,
    // create a text interface to allow a user to add employee names to a department in a company.
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    //
    let mut s = String::new();
    let mut v = Vec::new();
    loop {
        println!("Input string:");
        io::stdin().read_line(&mut s).expect("Must a string");
        let tmp = s.clone();
        for w in tmp.split_whitespace() {
            v.push(w.to_string());
        }
        if v.len() == 4 {
            let value = v[1].to_string();
            let key = v[3].to_string();

            // add value to a vector of values if key does not exists
            // use Entry enum
            match map.entry(key) {
                Entry::Vacant(e) => { e.insert(vec![value]); },
                Entry::Occupied(mut e) => { e.get_mut().push(value); },
            }
            break;
        } else {
            println!("Please input follow the format!");
        }
    }
}

fn main() {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for _ in 1..5 {
        create_employee(&mut result);
    }

    println!("result: {:?}", result);
}
