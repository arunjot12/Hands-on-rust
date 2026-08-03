use std::{sync::RwLock, thread};
use std::sync::Arc;

fn main() {
    
   let a = Arc::new(RwLock::new(10)); 
    let b = Arc::clone(&a);
    let c = Arc::clone(&a);

   thread::spawn(move ||{
   let b = b.read();
   });
   thread::spawn(move ||{
   let mut b =  c.write().unwrap();
   *b += 200;
   });

   println!("{:?}", a);
}
