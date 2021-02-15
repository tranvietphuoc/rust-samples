use std::collections::HashMap;

fn main() {
    // storing keys with associated values in hash maps

    // Hash maps have less support from the standard library, there's no built-in macro to
    // construct them
    // Just like vectors, hashmaps store their data on heap.
    // Like vectors, hashmaps are homogeneous: all keys must have the same type,
    // and all values must have the same type too
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // another way to construct a hasmap is by using iterators and collect method
    // on a vector of tuples

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores1: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores1);
    // the type annotation HashMap<_, _> is needed
    // Because it's possible to collect into many different data structures
    // and Rust doesn't know which you want unless you specify.
    //
    // hasmap and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);  // field_name and field_value are invalid at this point

    // accessing values in hasmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);  // the result will be wrapped by Some, because get returns an Option<&V>.
    //if there's no value for that key in the hasmap, get will return None

    // we can iterate each key/value pair in a hash map in a similar manner as we do with vectors.
    for (key, value) in &scores1{
        println!("{}: {}", key, value);
    }

    // Updating HashMap
    // 1. Using insert method using insert() method
    // 2. Only inserting a value if the key has no value using or_insert() method with entry() method
    scores1.entry(String::from("Yellow")).or_insert(60);
    scores1.entry(String::from("Blue")).or_insert(60);
    // or_insert method on Entry is defined to return a mutable reference
    // to the value for the corresponding Entry key if that key exists.
    // and if not, inserts the parameter as the new value for this key and returns a mutable
    // reference to the new value
    //
    // 3. Update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // returns a mutable reference here
        *count += 1;
    }
    println!("{:?}", map); // {"hello": 2, "world": 1, "wonderful": 1}
    // or_insert returns a mutable reference &mut V to the value for this key.
    //
    // Hashing Function, by default, HashMap uses a cryptographically strong,
    // hashing function can provide resistance to Denial of Service (Dos) attack. It's not a
    // fastest hashing algorithm, but better security.
    // To switch to another hashing, we can try BuildHasher trait
}
