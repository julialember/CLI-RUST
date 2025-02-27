use std::{collections::HashMap, env::args};

use programm::Command;
use programm::{Echo, Grep, Ls};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() > 1{
        let map: HashMap<String, Box<dyn Fn()>> = HashMap::from(
            [
                ("echo".to_string(),  Box::new(|| Echo::new(&args).task()) as Box<dyn Fn()>),
                ("grep".to_string(), Box::new(|| Grep::new(&args).task())),
                ("ls".to_string(), Box::new(|| Ls::new(&args).task()))
            ]
        );
        if let Some(ts) = map.get(args[1].trim()){
            ts();
        } else {println!("command: '{}' not found!", args[1])}
    } else {println!("cant get arguments!")}
    
}


