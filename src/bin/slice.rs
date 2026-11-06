fn main() {
    let s = String::from("dhiraj shrestha");
    let len = s.len();
    let part1 = &s[..]; //entire string is referenced
    let part2 = &s[1..2];
    let part3 = &s[1..=7];
    let part4 = &s[..];
    let part5 = &s[..len];

    println!("{2}, {1}, {0}, {4}, {3}", part3, part2, part1, part4, part5);
    let str = first_word(&s);
    println!("the first word is {str}");
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
//this is very complex and only return index to resolve this we can use string slice
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
