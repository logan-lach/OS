
use std::thread;
use std::time::Duration;

/* 

Threading practice (Needed for OS)

What this does is simulate scheduling within rust. 

fn runner() {
    for i in 1..10 {
        println!("{}", i);
    }
}
fn main() {

    let mut children = vec![];
    for _ in 1..10 {
        children.push(thread::spawn(|| { 
            runner() }));
    }

    for child in children {
        child.join();
    }
}
