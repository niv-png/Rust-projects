//goals: action, and then item
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify action"); //returns the rgument that the program has started with, nth() iterator
    let item = std::env::args().nth(2).expect("Please specify an item"); // expect() : Option() enum that will return the value by the user, if not present,will terminate the program, returning the provided message: Option enum because the value it either there or not

    let mut todo = Todo::new().expect("Initialization of db failed");

    println!("{:?},{:?}", action, item);

    // let mut todo = Todo {
    //     map: HashMap::new(),
    // };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error has occured: {}", why),
            },
        }
    }
}

struct Todo {
    // use rust built in HashMap to store key -  val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        //defining a fn that will either return Result or io:Error
        let mut f = std::fs::OpenOptions::new() //here we talk about how to open the file by define various openoptions - create(true) flag will create file if not already present
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?; //reads all the bytes in the file and appends them to content string cause muttable ref
        let map: HashMap<String, bool> = content //here we convert string file into HashMap
            .lines() //it will iterate each entry of the file
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>()) //split our lines on the tab character
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect(); //collects the iterations into a collection
        Ok(Todo { map })
    }

    //here fn is function that is defined within a struct and its first parameter is always self
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }

        std::fs::write("db.txt", content)
    }
    //here we add a complete action to toggle them completed or not
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            //muttable reference to the value of key or None if the value is absent in collection
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
