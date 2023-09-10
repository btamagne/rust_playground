fn main() {
    let s1 = String::from("Hello there");
    let s2 = "General Kenobi";

    let first1 = first_word(&s1);
    let first2 = first_word(&s2);

    println!("The first word in s1 is: {first1}");
    println!("The first word in s2 is: {first2}");
}

fn first_word(s: &str) -> &str { // str works for both String and literals! (a "deref coercion")
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
