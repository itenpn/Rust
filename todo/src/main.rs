extern crate chrono;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Objective{
    title: String,
    path: String,
    due: String,
}

fn main() {
   run();
}

fn run(){
    let content = read_file();
    let now = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!("TODO");
    println!("Date: {}", now);
    println!("Objectives\n");
    for i in content{
        println!("Title: {}", i.title);
        println!("Path to File: {}", i.path);
        println!("Due Date: {}", i.due);
        println!("\n");
    }
}

fn read_file() -> Vec<Objective>{
    let f = File::open("todo.txt").unwrap();
    let file = BufReader::new(&f);
    let mut t:String = String::new();
    let mut p:String = String::new();
    let mut d:String = String::new();
    let mut x:Vec<Objective> = Vec::new();
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        let mut z = num % 3;
        match z {
            0 => t = l,
            1 => p = l,
            _ => {
                    d = l;
                    x.push(
                        Objective{
                            title: t.clone(),
                            path: p.clone(),
                            due: d.clone()
                        });
                }
        }
    }
    x
}
