use std::collections::HashMap;
use std::ffi::CString;
use std::process::Command;
use std::{fmt::Error, io, path::PathBuf};
use std::path::{self, Path};

use libc::{c_char, ENETUNREACH};

fn main(){

    let vars  = std::env::vars().into_iter().collect(); 
    dbg!(&vars);

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        run_process(&vars, &line).unwrap();
    }
}

fn run_process(vars: &HashMap<String,String> , commnad: &str) -> Result< () , () > {
    let command : Vec<&str> = commnad.split(" ").collect();
    let bin =  match find_binary(commnad[0], &vars["PATH"]){
        Ok(b) => b , 
        Err(err) => {
            println!("Failed to find {}" , command[0]);
            return Err(());
        }
    };

    use std::os::raw::c_char;
    match  unsafe { libc::fork()} {
        -1 => {
            panic!("failed to start child process");
        }
        0 => {
            let pathname = &bin;
            let argv_owned : Vec<CString> = command.iter().map(|p| CString::new(*p).unwrap()).collect();
            let argv : Vec <*const c_char > = argv_owned.iter().map(|o| o.as_ptr()).collect();
            let argv: *const *const c_char = argv.as_ptr();
            unsafe {};
        }
        child_pid =>{
            println!( "hello chid is {child_pid} ");
        }        
    }
    println!("{:?}", bin);
    std::process::exit(0);
}


fn find_binary(commnad: &str , path: &str) -> Result< PathBuf ,  std::io::Error> {
    fn search (command :&str , path:&Path )-> Result<() , std::io::Error> {
        for entry in std::fs::read_dir(path)? {
            if let Ok(entry) = entry{
                if let Ok(met) = entry.metadata(){
                    if met.is_file() || met.is_symlink() {
                        if let Some(name) = entry.path().file_name(){
                            if name == command{
                                if met.is_symlink(){
                                    panic!("Running symlinks not supported");
                                }
                                return  Ok(());
                            }
                        }
                    }
                }
            }

        }
        Err(std::io::ErrorKind::NotFound.into())
    }
    if let Ok(mut dir) = std::env::current_dir(){
        if let Ok(()) = search(commnad, &dir){
            dir.push(commnad);
            return  Ok(dir);
        }
    }
    for entry in path.split(":"){
        let mut path = PathBuf::from(entry);
        if let Ok(()) = search(commnad, &path){
            path.push(commnad);
            return  Ok(path);
        }
    }
    Err(std::io::ErrorKind::NotFound.into())
}
 