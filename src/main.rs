use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
//use std::slice::range;
use std::io::{Write, BufReader, BufRead, Error};

struct schedule {

}

impl schedule {
    pub fn initialize() -> schedule {
        schedule {}
    }

    pub fn read_from_file(&mut self)  -> Result<(), Error>  {
        let mut file = std::fs::File::open("src/foo.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut j = 0;
        let mut k = 0;
        let mut hours = 0;

        for line in contents.lines() {
            j = self.first_word(& line.to_string(), &0);
            if line[0..j].to_string().trim() == "High" { 
                println!("Must Do!");
            }
            k = self.last_word(&line.to_string());
            hours += match line[k..line.len()].to_string().trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("Task:{}", line[j..k].to_string());
            println!("Time: {}", hours);
            
            j = 0;
            k = 0;
        }
        println!("All hours total: {}", hours);
        Ok(())
    }

    pub fn first_word(&self, line: &String, start_slice: &i32) -> usize {
        let bytes = line[*start_slice as usize..line.len() - 1].as_bytes();

        for (i, &letter) in bytes.iter().enumerate() {
            //println!("{:b}", &letter);
            if letter == b' ' || letter == b'\\'{
                return i + *start_slice as usize;
            }
        }
        line.len()
    }

    pub fn last_word(&self, line: &String) -> usize {
        let bytes = line.as_bytes();

        for (i, &letter) in bytes.iter().enumerate().rev() {
            if letter == b' ' {
                return i;
            }
        }
        line.len()
    }

    pub fn write_to_file(&self) -> std::io::Result<()>{
        let mut input = String::new();

        let mut file = File::create("src/foo.txt")?;
        println!("Priorities: High, Medium-High, Medium, Medium-Low, Low");
        println!("Priority Task Duration\n");
    
        loop {
            io::stdin().read_line( &mut input).expect("Failed to read line");
            if input.len() >= 5 && input[input.len() - 5..input.len()].to_string().to_lowercase().trim() == "end" {
                break;
            }
            fs::write("src/foo.txt", &input).expect("Unable to write to file");
        }

        Ok(())
    }

}

fn main() {
    let mut schedule = schedule::initialize();
    let result = schedule.write_to_file();
    let output = schedule.read_from_file();
}

//class time
//priority Task time 
/*    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5*/