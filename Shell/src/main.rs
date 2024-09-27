use std::io;

fn main(){

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        println!( "{}" , line);
    }
}


 