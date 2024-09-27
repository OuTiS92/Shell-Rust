use std::io;

fn main(){

    let args: Vec<String> = std::env::args().collect(); 
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        println!( "{}" , line);
    }
}




 