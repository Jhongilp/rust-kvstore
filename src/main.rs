use std::collections::HashMap;


fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("The key is {key} and value is {value}");

    let contents = format!("{key}\t{value}\n");
    std::fs::write("kv.db", contents).unwrap();
}


struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        Database { map: HashMap::new(),  }
    }
}