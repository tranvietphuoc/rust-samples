// Problem:
// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay”
// added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
//
fn check_vowels(s: &str) -> bool {
    let vowels = vec!['a', 'i', 'e', 'o', 'u'];
    let mut list_of_string = vec![];
    for c in s.chars(){
        list_of_string.push(c);
    }
    for i in &vowels {
        if list_of_string[0] == *i {
            return true;
        }
    }
    return false;
}

fn convert_pig_latin(s: &str) -> String {
    let mut chars: Vec<char> = vec![];
    let mut result: Vec<char> = vec![];
    let mut tmp: String;
    // turn a string into a list
    for c in s.chars() {
        chars.push(c);
    }
    // check vowels
    if check_vowels(s){
        chars.push('-');
        chars.push('h');
        chars.push('a');
        chars.push('y');
        tmp = chars.into_iter().collect();  // turn vector into a String
        return tmp;
    }
    let first_char = &chars[0];
    // slice the result vector from the second char of chars vector
    for (index, c) in chars.iter().enumerate() {
        if index == 0 {
            continue;
        } else {
            result.push(*c);
        }

    }
    result.push('-');
    result.push(*first_char);
    result.push('a');
    result.push('y');
    tmp = result.into_iter().collect();
    return tmp;
}


fn main () {
    println!("result: {:?}", convert_pig_latin("apple"));
    println!("result: {:?}", convert_pig_latin("first"));

}
