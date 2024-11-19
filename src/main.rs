use std::io;
fn main(){
    println!("welcome to iMean! Type exit to quit");
    let mut mood = "neutral";

    loop{
        println!("You: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase();
        if input == "exit"{
            println!("iMean: Goodbye!");
            break;
        }
        let response = match input.as_str(){
            "Hey Abadie?" => {
                mood = "curious";
                
            }
        }
    }
}