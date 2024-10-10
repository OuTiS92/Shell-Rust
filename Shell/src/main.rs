use std::collections::HashMap;
use std::ffi::CString;
use std::process::Command;
use std::{fmt::Error, io, path::PathBuf};
use std::path::{self, Path};

use libc::{c_char, CS, ENETUNREACH};

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
            let pathname = CString::new(bin.to_str().expect("only utf8")).unwrap();
            let argv_owned : Vec<CString> = command.iter().map(|p| CString::new(*p).unwrap()).collect();
            let mut argv : Vec <*const c_char > = argv_owned.iter().map(|o| o.as_ptr()).collect();
            argv.push(std::ptr::null());
            let argv: *const *const c_char = argv.as_ptr();
            
            
            let envp_owned : Vec<CString> = vars.iter().map(|(k,v)| {
                let mut both= k.clone();
                both.push_str("=");
                both.push_str(&v);
                CString::new(both).expect("null byte not allowed in env  string")
            })
                .collect();
            let mut envp : Vec <*const c_char > = envp_owned.iter().map(|o| o.as_ptr()).collect();
            envp.push(std::ptr::null());
            let  envp: *const *const c_char = envp.as_ptr();
            
            let res = unsafe {
                libc::execve(pathname.as_ptr(), argv, envp)
            };
            let err = std::io::Error::last_os_error();
            println!( "ERROR! {}" , err);
            std::process::exit(0);
        }
        child_pid =>{
            println!( "hello chid is {child_pid} ");
            let code= unsafe {
                libc::wait(child_pid);
            };
            Ok(())
        }    
            
    }
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
 