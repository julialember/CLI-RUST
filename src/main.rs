use std::{collections::HashMap, env::args, fs, io::Error};

fn main() {
    let args = args().collect::<Vec<String>>();
    let map: HashMap<String, Box<dyn Fn()>> = HashMap::from(
        [
            ("echo".to_string(),  Box::new(|| Echo::new(&args).task()) as Box<dyn Fn()>),
            ("grep".to_string(), Box::new(|| Grep::new(&args).task())),
            ("ls".to_string(), Box::new(|| Ls::new(&args).task()))
        ]
    );
    if let Some(ts) = map.get(args[1].trim()){
        ts();
    } 
}

trait Command<'a, C> where C: 'a{
    fn task(&self);
    fn new(args: &'a Vec<String>) -> C;
}
struct Grep<'a>{
    args: &'a Vec<String>,
}
impl<'a> Command<'a, Self> for Grep<'a>{
    fn new(args: &'a Vec<String>) -> Self {
        Grep { args: &args }
    }
    fn task(&self) {
        for i in open_file(self.args[2].trim())
            .lines().filter(|line| line.contains(self.args[3].trim())){
                println!("{}", i)
            }
    }
}
struct Ls<'a>{
    args: &'a Vec<String>
}
impl<'a> Command<'a, Self> for Ls<'a> {
    fn task(&self){
        match read_dir(&self.args[2]){
            Ok(()) => (),
            Err(kind) => println!("error!: {}", kind)
        }
    }
    fn new(args: &'a Vec<String>) -> Ls<'_> {
        Ls { args: &args }        
    }
}

struct Echo<'a>{
    args: &'a Vec<String>
}

impl<'a> Command<'a, Self> for Echo<'a>{
    fn new(args: &Vec<String>) -> Echo<'_>{
        Echo{
            args: &args
        }
    }
    fn task(&self) {
        for i in  &self.args[2..]{
            print!("{} ", i)
        }
    }
}

fn open_file(name: &str) -> String{
    fs::read_to_string(name).unwrap()
}

fn read_dir(name: &String) -> Result<(), Error>{
    let dir =  fs::read_dir(name.trim())?;
    for i in dir{
        let pat = i.unwrap().path();
        println!("{}", pat.display());
        if pat.is_dir(){
            read_dir(&pat.display().to_string())?;
        }
        drop(pat);
    }
    Ok(())
}