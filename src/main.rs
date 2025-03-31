use std::io;

fn main() {
    let mut playing:bool = true;
    println!(r" ____  _____ ____  _____    _  _____      _    _____ _____ _____ ____  
|  _ \| ____|  _ \| ____|  / \|_   _|    / \  |  ___|_   _| ____|  _ \ 
| |_) |  _| | |_) |  _|   / _ \ | |     / _ \ | |_    | | |  _| | |_) |
|  _ <| |___|  __/| |___ / ___ \| |    / ___ \|  _|   | | | |___|  _ < 
|_| \_\_____|_|   |_____/_/   \_\_|   /_/   \_\_|     |_| |_____|_| \_\
                                                                       
 __  __ _____ 
|  \/  | ____|
| |\/| |  _|  
| |  | | |___ 
|_|  |_|_____|
              
");
    println!("Type something and I will repeat it! press q to quit");

    while playing{
        let mut user_phrase = String::new();
        io::stdin()
            .read_line(&mut user_phrase)
            .expect("Failed to read line");

        let user_phrase = user_phrase.trim();

        match user_phrase {
            "q" => playing = false,
            _ => {
                println!("You said: \"{}\"", user_phrase);
                println!("\nSay something again, or press q to quit");
                }

        }
        
    }
    println!("Bye-bye")
}
