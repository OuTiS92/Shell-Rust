use std::collections::HashMap;
use std::ffi::CString;
use std::io::Write;
use std::path::Path;
use std::{io, path::PathBuf};
fn main() {
    let vars = std::env::vars().into_iter().collect();
    loop {
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[34mOuTiS92 ~\x1b[0m").unwrap();
        stdout.flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        run_process(&vars, &line).unwrap();
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Command<'c>(Vec<&'c str>);

impl<'c> Command<'c> {
    pub fn new(command: &'c str) -> Self {
        assert!(!command.is_empty(), "Command can not be empty! ");
        Self(command.split(" ").collect())
    }
    pub fn bin_path(&self) -> &str {
        self.0[0]
    }
    pub fn iter(&self) -> std::slice::Iter<'_, &str> {
        self.0.iter()
    }
}

impl<'c> std::fmt::Display for Command<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in &self.0 {
            f.write_fmt(format_args!("{}", p))?;
            f.write_str(", ")?;
        }
        Ok(())
    }
}

fn run_process(vars: &HashMap<String, String>, command: &str) -> Result<(), ()> {
    if command.is_empty() {
        shell_exit();
    }
    let command = Command::new(command);
    match run_shell_internals(&command) {
        Ok(()) => {
            return Ok(());
        }
        Err(()) => {}
    }
    let bin = match find_binary(&command, &vars["PATH"]) {
        Ok(b) => b,
        Err(_err) => {
            println!("Failed to find {}", command);
            return Err(());
        }
    };
    use std::os::raw::c_char;
    match unsafe { libc::fork() } {
        -1 => {
            panic!("failed to start child process");
        }
        0 => {
            let pathname = CString::new(bin.to_str().expect("only utf8")).unwrap();
            let argv_owned: Vec<CString> =
                command.iter().map(|p| CString::new(*p).unwrap()).collect();
            let mut argv: Vec<*const c_char> = argv_owned.iter().map(|o| o.as_ptr()).collect();
            argv.push(std::ptr::null());
            let argv: *const *const c_char = argv.as_ptr();

            let envp_owned: Vec<CString> = vars
                .iter()
                .map(|(k, v)| {
                    let mut both = k.clone();
                    both.push_str("=");
                    both.push_str(&v);
                    CString::new(both).expect("null byte not allowed in env  string")
                })
                .collect();
            let mut envp: Vec<*const c_char> = envp_owned.iter().map(|o| o.as_ptr()).collect();
            envp.push(std::ptr::null());
            let envp: *const *const c_char = envp.as_ptr();

            let _res = unsafe { libc::execve(pathname.as_ptr(), argv, envp) };
            let err = std::io::Error::last_os_error();
            println!("ERROR! {}", err);
            std::process::exit(0);
        }
        child_pid => {
            println!("hello chid is {child_pid} ");
            let mut exit_code = 0;
            let _code = unsafe {
                libc::waitpid(child_pid, &mut exit_code, 0);
            };
            if exit_code != 0 {
                println!("Exit whith {}", exit_code);
            }
            Ok(())
        }
    }
}

fn run_shell_internals(command: &Command) -> Result<(), ()> {
    let bin = command.bin_path();
    match bin {
        "exit" => {
            shell_exit();
        }
        "cd" => {
            let path = command.0[1];
            let path = if !path.starts_with("/") {
                let mut cwd = std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .expect("No Null bytes in cd please")
                    .to_owned();
                cwd.push_str("/");
                cwd.push_str(path);
                println!(">changing to {cwd}");
                let a = std::fs::canonicalize(cwd).unwrap();
                CString::new(a.to_str().unwrap()).unwrap()
            } else {
                CString::new(path).unwrap()
            };
            match unsafe { libc::chdir(path.as_ptr()) } {
                0 => {}
                _ => {
                    let err = std::io::Error::last_os_error();
                    println!("Failed to cd {}", err);
                }
            }
            Ok(())
        }
        "export" => {
            unimplemented!()
        }
        _ => Err(()),
    }
}

fn find_binary(command: &Command, path: &str) -> Result<PathBuf, std::io::Error> {
    fn search(target: &str, path: &Path) -> Result<(), std::io::Error> {
        for entry in std::fs::read_dir(path)? {
            if let Ok(entry) = entry {
                if let Ok(met) = entry.metadata() {
                    if met.is_file() || met.is_symlink() {
                        if let Some(name) = entry.path().file_name() {
                            if name == target {
                                if met.is_symlink() {
                                    panic!("Running symlinks not supported");
                                }
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        Err(std::io::ErrorKind::NotFound.into())
    }
    let target = command.bin_path();
    if let Ok(mut dir) = std::env::current_dir() {
        if let Ok(()) = search(target, &dir) {
            dir.push(target);
            return Ok(dir);
        }
    }
    for entry in path.split(":") {
        let mut path = PathBuf::from(entry);
        if let Ok(()) = search(target, &path) {
            path.push(target);
            return Ok(path);
        }
    }
    Err(std::io::ErrorKind::NotFound.into())
}

fn shell_exit() -> ! {
    std::process::exit(0);
}
