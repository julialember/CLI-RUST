use std::{fs, io::Error};

pub trait Command<'a, C> where C: 'a{
    fn task(&self);
    fn new(args: &'a Vec<String>) -> C;
}

pub struct Grep<'a>{
    args: &'a Vec<String>,
}
impl<'a> Command<'a, Self> for Grep<'a>{
    fn new(args: &'a Vec<String>) -> Self {
        Grep { args: &args }
    }
    fn task(&self) {
        if self.args.len() < 4{
            println!("not enought arguments!")
        }else {
        match open_file(self.args[2].trim()) {
            Ok(text) => {
                for i in text.lines().filter(|line| line.contains(self.args[3].trim())){
                    println!("{}", i)
                }}
            Err(kind) => println!("error!: {}", kind),
        }
    }}
}
pub struct Ls<'a>{
    args: &'a Vec<String>
}
impl<'a> Command<'a, Self> for Ls<'a> {
    fn task(&self){
        if self.args.len() < 3{
            println!("not enought arguments!");
        }else{
        match read_dir(&self.args[2]){
            Ok(()) => (),
            Err(kind) => println!("error!: {}", kind)
        }}
    }
    fn new(args: &'a Vec<String>) -> Self{
        Ls { args: &args }        
    }
}

pub struct Echo<'a>{
    args: &'a Vec<String>
}

impl<'a> Command<'a, Self> for Echo<'a>{
    fn new(args: &'a Vec<String>) -> Self{
        Echo{
            args: &args
        }
    }
    fn task(&self) {
        if self.args.len() < 3{
            println!("not enought elements!")
        }else{
        for i in  &self.args[2..]{
            print!("{} ", i)
        }}
    }
}

fn open_file(name: &str) -> Result<String, Error>{
    fs::read_to_string(name)
}


fn read_dir(name: &String) -> Result<(), Error>{
    let dir =  fs::read_dir(name.trim())?;
    for i in dir{
        match i {
            Ok(pat) => {
                let pat = pat.path();
                println!("{}", pat.display());
                if pat.is_dir(){
                read_dir(&pat.display().to_string())?;
                drop(pat);
                }}
            Err(kind) => println!("error!:{}", kind),
        }
    }
    Ok(())
}
