enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        String::from("Json String") 
    }
}

fn main() {
    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace {
        from: String::from("kid"),
        to: String::from("rogit"),
    };

    let json_string = cmd.serialize();
    println!("{}", json_string);
}


// emi,s are power ful as they represent Variance  they can have data also they add impl blocks  