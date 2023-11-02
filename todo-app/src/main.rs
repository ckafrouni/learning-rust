#![allow(dead_code)]
mod todo;

use my_lists::{Vector, LinkedList};
use todo::Todo;

fn main() {

    let mut todos = Vector::new();
    todos.push(Todo::new("Buy milk".to_string()));
    todos.push(Todo::new("Buy eggs".to_string()));
    todos.push(Todo::new("Buy bread".to_string()));

    todos.get_mut(0).unwrap().toggle();

    for todo in todos.iter() {
        println!("{}", todo);
    }

    let mut todos = LinkedList::new();
    todos.push(Todo::new("Buy milk".to_string()));
    todos.push(Todo::new("Buy eggs".to_string()));
    todos.push(Todo::new("Buy bread".to_string()));

    todos.get_mut(0).unwrap().toggle();

    for todo in todos.iter() {
        println!("{}", todo);
    }


}
