
    fn main() {
        let name = String::from("Alice");
        let num = 100;

        match name.as_str() {
            "Bob" => println!("I am Bob"),
            "Alice" => println!("I am Alice"),
              _ => println!("Default"),
        }
        match num {
            n => println!("I am Alice"),
            100 => println!("I am Bob"),
        }
    }
    

