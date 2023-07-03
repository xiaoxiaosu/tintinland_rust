pub fn print_big_a_to_z() {
    for c in b'A'..=b'z'{
        println!("ASCII:{},char:{}",c as u8, c as char);
    }
}