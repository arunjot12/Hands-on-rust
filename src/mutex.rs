use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
    
    let mut counter = Arc::new(Mutex::new(5));
    let mut handles  = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move ||{
            let mut lock = counter.lock().unwrap();
            *lock +=1;
        }))
    }

    for h in handles{
        h.join().unwrap();
    }
    println!("{:?
    }",counter);

}
