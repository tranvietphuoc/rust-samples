fn main() {
    // rust has only one string type in the core language
    // which is the string slice `str` that is usually seen in it borrowed from &str
    // string literal <=> string slice
    // the String type is growable, mutable, owned, utf-8 encode string type
    // strings in Rust <=> String type and &str type
    // create a new String
    let mut s = String::new();
    let data = "test";
    s = data.to_string();
    // or we can write: let s = "test".to_string();
    let s1 = String::from("test");  // we can also use the String::from()
    // String::from and .to_string do the same things
    // update the string
    // appending using push_str and push
    let mut s2 = String::from("foo");
    s2.push_str("bar");  // the method push_str take a string slice
    let s3 = "bar";
    s2.push_str(s3);
    println!("s3 is {}", s3);  // this code will work
    s2.push('s');  // push method take a single character as parameter

    // concatenation with + operator and format! macro
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;  // s1 has been moved, s2 still can be use because it use reference
    // the reason of s1 no longer valid after addition and s2 use a reference
    // because the signature of the method that gets called whe we use + operator.
    // the + operator uses add method, the signature look like
    // fn add(self, s: &str) -> String {//}
    // in standard library, add is defined using generics
    // we can only add a &str to a String. we can't add two String together
    // the codes above use &s2 (type &String, not &str) the compiler coerce the &String
    // argument into &str.
    // when we call add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    //
    // for more complicated string combining, we can use format! macro
    let s1 = String::from("test");
    let s2 = String::from("another test");
    let s3 = String::from("anoter test again");
    let s = format!("{} - {} - {}", s1, s2, s3);
    // format! macro works in the same ways as prinln!
    // but instead of printing output to the screen, it returns a String with the content.
    //
    // Index into Strings

    let s1 = String::from("hello");
    // let h = s[0]; // error String type cannot be indexed by integer
    let hello = "Здравствуйте";
    // Rust doesn't allow us to index in to a String to get a character.
    // because: indexing operations are expected to always take O(1).
    // Rust would have to walk through the contents from the begining to the index
    // to determine how many valid character there were.
    //
    // Indexing into a string is often a bad idea because it's not clear what the return
    // type of string indexing operation: a byte value, a string clice, a character or a grapheme cluster
    // you can use [] with a range instead a single number. a range must greater than 1
    //
    // Methods for iterating over Strings

    for c in hello.chars(){  // individual Unicode scalar values
        println!("{}", c);
    }

    for c in hello.bytes(){  // for raw bytes
        println!("{}", c);
    }
    // => Strings are not so simple

}
