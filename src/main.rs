use std::env;
mod monitor_brightness;

fn get_command() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("You need to send a command");
        return String::from("");
    }

    let command = &args[1];
    return command.to_string();
}

fn main() { 
    // let command = get_command();
    
    // match command.as_str() {
    //     "up" => print!("Brightness up!"),
    //     "down" => print!("Brightness down!"),
    //     _ => print!("Invalid command!"),
    // }

    monitor_brightness::MonitorBrightness::new();
}
