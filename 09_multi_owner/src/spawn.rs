
// fn main() {
//     let  arr = vec![1];
//
//     std::thread::spawn(move|| {
//         println!("{:?}", arr);
//     });
// }

use std::sync::Arc;
fn main() {
    let s = Arc::new("rust rocks!");
    let s1 = s.clone();

    let handler = std::thread::spawn(move || {
        println!("thread: {:?}", s1);
    });
    println!("main: {:?}", s);
    
    handler.join().unwrap();
}