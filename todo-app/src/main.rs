#![allow(dead_code)]
mod todo;

use my_lists::{Vector, LinkedList};
use todo::Todo;

fn main() {

    let mut todos = Vector::new();
    todos.push(Todo::new("Buy milk"));
    todos.push(Todo::new("Buy eggs"));
    todos.push(Todo::new("Buy bread"));

    todos.get_mut(0).unwrap().toggle();

    for todo in todos.iter() {
        println!("{}", todo);
    }

    let mut todos = LinkedList::new();
    todos.push(Todo::new("Buy milk"));
    todos.push(Todo::new("Buy eggs"));
    todos.push(Todo::new("Buy bread"));

    todos.get_mut(0).unwrap().toggle();

    for todo in todos.iter() {
        println!("{}", todo);
    }


}
