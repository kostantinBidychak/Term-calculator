use std::io;

macro_rules! calculate_to_fahrenheit {
    ($c:expr) => {
        ($c as f32 *1.8) + 32.0
    };
}
fn main() {
    let mut celsius = String::new();
    println!("Write num");
    io::stdin().read_line(&mut celsius).expect("eror");
   let input:f32 = celsius.trim().parse().expect("eror");
   let fahrenheits = calculate_to_fahrenheit!(input);
   println!("temperature in Fahrenheit {}",fahrenheits);
     is_freezing(input);
}

fn is_freezing(c: f32) -> bool
{
    let is_freeze = c < 0.0;
    println!("{}",is_freeze);
    return is_freeze;
}