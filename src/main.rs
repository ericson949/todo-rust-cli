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
    fn list(&self) ->
                   (impl Iterator<Item=&String>, impl Iterator<Item=&String>){
        (
            self.items.iter().filter(|(_, v)| **v == true).map(|(k, _)| k),
            self.items.iter().filter(|(_, v)| **v == false).map(|(k, _)| k)
        )
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

    #[test]
    fn list_items(){
        let mut todo =  TodoList::new();
        todo.add(String::from("Something to do"));
        todo.add(String::from("Something else to do"));
        todo.add(String::from("Something done"));
        todo.mark(String::from("Something done"), false);

        let (todo_items, done_items) = todo.list();

        let todo_items: Vec<String> = todo_items.cloned().collect();
        let done_items: Vec<String> = done_items.cloned().collect();

        assert!(todo_items.iter().any(|e| e =="Something to do"));
        assert!(todo_items.iter().any(|e| e =="Something else to do"));
        assert_eq!(todo_items.len(), 2);

        assert!(done_items.iter().any(|e| e =="Something done"));
        assert_eq!(done_items.len(), 1);
    }
}
