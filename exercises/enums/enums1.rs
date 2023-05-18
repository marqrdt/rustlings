// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32 ,i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Echo")));
    println!("{:?}", Message::Move{x: 20, y: 30});
    println!("{:?}", Message::ChangeColor(0, 255, 255));
}
