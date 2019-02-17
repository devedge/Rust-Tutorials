#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    let test = match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
            1
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
            2
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
            3
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
            4
        }
    };

    dbg!(test);
}
