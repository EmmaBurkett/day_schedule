use std::io::{prelude::*, self};
use std::{fs};
//use std::slice::range;
use std::io::Error;

struct TodoList {
    priority: i32,
    task: String,
    duration: f32
}
struct Schedule {
    file: String,
    contents: String,
    to_do: Vec<TodoList>
}

impl Schedule {
    pub fn initialize(file: String, contents: String, to_do: Vec<TodoList>) -> Schedule {
        Schedule {file, contents, to_do}
    }
    //Gets file and converts contents into a string
    pub fn file(&mut self)  -> Result<(), Error>  {
        //set file name
        self.file = "src/foo.txt".to_string();
        
        //user writes to file
        let _result = self.write_to_file();

        //open file
        let mut file = std::fs::File::open(&self.file).unwrap();

        //put file data into a string
        file.read_to_string(&mut self.contents).unwrap();

        //put file data into a vector
        self.read_from_file();

        //organizes vector by priorities
        self.prioritize();

        //prints out a day plan
        self.day_plan();

        //return Result<(), Error>
        Ok(())
    }

    //User writes to_do list to file
    pub fn write_to_file(&mut self) -> std::io::Result<()>{
        let mut input = String::new();

        println!("Priorities: High, Medium-High, Medium, Medium-Low, Low");
        println!("Priority Task Duration\n");
    
        loop {
            io::stdin().read_line( &mut input).expect("Failed to read line");
            if input.len() >= 5 && input[input.len() - 5..input.len()].to_string().to_lowercase().trim() == "end" {
                break;
            }
            fs::write(&self.file, &input).expect("Unable to write to file");
        }

        Ok(())
    }


    //put to_do list into a vector
    pub fn read_from_file(&mut self)  {
        let mut _j = 0;
        let mut _k = 0;
        let mut _priority_string = String::new();
        let mut _priority = 0;
        let mut _task = String::new();
        let mut _duration:f32 = 0.0;
        for line in self.contents.lines() {
            _j = self.first_word(& line.to_string(), &0);
            _k = self.last_word(&line.to_string());
            _duration = match line[_k..line.len()].to_string().trim().parse::<f32>() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
            _task = line[_j.._k].to_string();
            _priority_string = line[0.._j].to_string();

            if _priority_string.to_lowercase().trim() == "high" {
                _priority = 4;
            } 
            else if _priority_string.to_lowercase().trim() == "medium-high" {
                _priority = 3;
            }
            else if _priority_string.to_lowercase().trim() == "medium" {
                _priority = 2;
            }
            else if _priority_string.to_lowercase().trim() == "medium-low" {
                _priority = 1;
            }
            else if _priority_string.to_lowercase().trim() == "low" {
                _priority = 0;
            } 
            
            
            let task = TodoList {
                priority: _priority,
                task: _task,
                duration: _duration
            };

            self.to_do.push(task);

            _j = 0;
            _k = 0;
        }
    }

    //get the first word in a string
    pub fn first_word(&self, line: &String, start_slice: &i32) -> usize {
        let bytes = line[*start_slice as usize..line.len() - 3].as_bytes();

        for (i, &letter) in bytes.iter().enumerate() {
            //println!("{:b}", &letter);
            if letter == b' ' {
                return i + *start_slice as usize;
            }
        }
        line.len()
    }

    //get the last word in a string
    pub fn last_word(&self, line: &String) -> usize {
        let bytes = line.as_bytes();

        for (i, &letter) in bytes.iter().enumerate().rev() {
            if letter == b' ' {
                return i;
            }
        }
        line.len()
    }

    pub fn prioritize(&mut self) {
        let mut _switch_priority = 0;
        let mut _switch_task = "".to_string();
        let mut _switch_duration:f32 = 00.0;
        
        for _j in 0..self.to_do.len() {
            for i in 0..self.to_do.len() {
                if i != self.to_do.len() - 1 && self.to_do[i].priority < self.to_do[i+1].priority {
                    
                    _switch_priority = self.to_do[i].priority;
                    _switch_task = self.to_do[i].task.clone();
                    _switch_duration = self.to_do[i].duration;

                    self.to_do[i].priority = self.to_do[i+1].priority;
                    self.to_do[i].task = self.to_do[i+1].task.clone(); //string does not auto clone!!
                    self.to_do[i].duration = self.to_do[i+1].duration;

                    self.to_do[i+1].priority = _switch_priority;
                    self.to_do[i+1].task = _switch_task;
                    self.to_do[i+1].duration = _switch_duration;

                }
            }
        }
    }

    pub fn day_plan(&self) {
        for j in 0..self.to_do.len() {
            println!("Priority: {}\nTask: {}\nDuration: {}", self.to_do[j as usize].priority, self.to_do[j as usize].task, self.to_do[j as usize].duration);
        }
    }

}

fn main() {
    let file_name = String::new();
    let contents= String::new();
    let _priority = 0;
    let _task = "".to_string();
    let _duration = 0.00;

    let to_do = Vec::new();
    let mut schedule = Schedule::initialize(
        file_name,
        contents,
        to_do
    );
    let _output = schedule.file();

}

//strike throughs
//new files
//7 day plan
//due dates instead of priorities
//accept different kinds of date formats
//allow data to be scrambled
//find duration even with different kinds of date formats based on surroundings
//write day plan to the file
//prioritize by date and duration
//option to stay one day ahead
//take different time units


//class time
//priority Task time 
/*    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5*/


















/*_j = self.first_word(& line.to_string(), &0);
            if line[0.._j].to_string().trim() == "High" { 
                println!("Must Do!");
            }
            _k = self.last_word(&line.to_string());
            hours += match line[_k..line.len()].to_string().trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("Task:{}", line[_j.._k].to_string());
            println!("Time: {}", hours);
            */