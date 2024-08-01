use std::collections::hash_map::Entry;
use std::collections::HashMap;
use clap::{Command, Parser};


#[derive(Parser)]
struct Cli{
    command: String,
    key:String
}

struct TodoList {
    // true = to do, false = done
    items: HashMap<String, bool>
}
impl TodoList {
    fn new () -> TodoList{
        let items: HashMap<String, bool> = HashMap::new();

        TodoList {items:items}
    }

    fn add(&mut self, key:String){
        if let Entry::Vacant(entry) = self.items.entry(key) {
            entry.insert(true);
        }
    }
    fn mark(&mut self, key: String, value: bool) -> Result<String, String> {
        let x = self.items.get_mut(&key).ok_or(&key)?;
        *x = value;

        Ok(key)
    }
}

fn main() {
    let args = Cli::parse();

    println!("{:?}, {:?}", args.command, args.key);
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn init_todo(){
        let todo = TodoList::new();
    }

    #[test]
    fn add_item(){
        let mut todo = TodoList::new();
        todo.add(String::from("Something to do"));
        assert_eq!(todo.items.get("Something to do"), Some(&true))
    }
    #[test]
    fn add_item_already_exist(){
        let mut todo =  TodoList::new();
        todo.add(String::from("Something to do"));
        todo.add(String::from("Something to do"));
        assert_eq!(todo.items.get("Something to do"), Some(&true));
        assert_eq!(todo.items.len(), 1)

    }
    #[test]
    fn add_item_does_not_change_existing(){
        let mut todo =  TodoList::new();
        todo.add(String::from("Something to do"));
        if let Some(x) = todo.items.get_mut("Something to do") {
            *x = false;
        }
        todo.add(String::from("Something to do"));
        assert_eq!(todo.items.get("Something to do"), Some(&false));
        assert_eq!(todo.items.len(), 1)
    }

    #[test]
    fn mark_item(){
        let mut todo =  TodoList::new();
        todo.add(String::from("Something to do"));
        todo.mark(String::from("Something to do"), false);
        assert_eq!(todo.items.get("Something to do"), Some(&false));
        assert_eq!(todo.items.len(), 1)
    }

    #[test]
    fn mark_item_does_not_exist(){
        let mut todo =  TodoList::new();
        assert_eq!(todo.mark(String::from("Something to do"), false),
            Err(String::from("Something to do"))
        );
    }
}
