use my_lists::{Vector, LinkedList};

fn main() {
    println!("Hello, world!");

    let mut v = Vector::<u8>::new();

    for _ in 0..20 {
        v.push(rand::random());
        println!("{}/{}", v.length(), v.capacity());
    }

    println!("{:?}", v);

    for _ in 0..20 {
        v.pop();
        println!("{}/{}", v.length(), v.capacity());
    }

    println!("{:?}", v);

    let mut l = LinkedList::<u8>::new();

    for _ in 0..20 {
        l.push(rand::random());
        println!("{:?}", l);
    }

    for _ in 0..20 {
        l.pop();
        println!("{:?}", l);
    }
}
