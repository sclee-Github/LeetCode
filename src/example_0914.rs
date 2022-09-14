/*
fn r#return() -> i8 {
    println!("Here is your number");
    100
}


fn main() {
    let my_number = r#return();
    println!("{}", my_number);
}
*/

/*
fn main() {
    println!("{:?}", b"This is test sentence.");
    println!("{:?}", br"This is test sentence.");

    println!("{:X}", '행' as u32);
    println!("{:X}", 'H' as u32);

    println!("\u{D589}, \u{48}");

    let number = 100;
    let number_ref = &number;

    println!("{}", number_ref); // print 100
    println!("{:p}", number_ref); // print pointer address

    let my_number = 200;

    println!("Binary : {:b}, Octal : {:o}, Hexadecimal : {:x}", my_number, my_number, my_number);

    let first_str = "First";
    let sec_str = "Second";
    let third_str = "Third";

    println!("String Exchange From {2}, {1}, {0} to {0}, {1}, {2}", first_str, sec_str, third_str);
}
 */

/*
 fn main() {
    println!("{rust} is difficult and {python} is also difficult,
but, {rust} is fun to learn.", rust = "Rust", python = "Python");

    let letter = "a";
    println!("{: ^10}", letter);
    println!("{: <10}", letter);
    println!("{: >10}", letter);

    let title = "TODAY'S NEWS";
    println!("{: ^30}", title);

    let bar = "|";
    println!("{:-<15}{:+>15}", bar, bar);

    let a = "Seoul";
    let b = "Tokyo";

    println!("{city1:-<15}{city2:+>15}", city1 = a, city2 = b);
}
 */

/*
 fn main() {
    let my_name = "Shincheol".to_string();
    let other_name = String::from("shincheol2");

    println!("{}", my_name);
    println!("{}", other_name);

    let mut my_other_name = "Shincheol".to_string();
    my_other_name.push('!');

    println!("{}", my_other_name);
}
 */

/*
 fn main() {
   let mut my_name = "Shincheol".to_string();
   my_name.push('!');
   my_name.push_str(" and my first name is Lee");
   println!("My name is {}", my_name);
   println!("Capacity of my name is {}", my_name.capacity());
   println!("Length of my name is {}", my_name.len());
}
 */
