pub fn print_a_to_big_z() {
    for c in (b'Z'..=b'a').rev() {
        println!("ASCII:{},char:{}",c as u8, c as char);
    }
}