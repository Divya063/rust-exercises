mod broadcast;

use broadcast::Broadcast;
use std::thread;


fn main() {
    
    let mut broadcaster = Broadcast::new();
    let receiver = broadcaster.subscribe();
    let receiver2 = broadcaster.subscribe();

    let receiver_thread_one = thread::spawn(move || {
        loop { 
            // unwrap can cause panic
            let message = receiver.recv();
            match message {
                Ok(message) => println!("Received from receiver: {}", message),
                Err(err) => {
                    println!("Error: {:?}", err); 
                    break;
                }
            }
        }
    });

    let receiver_thread_two = thread::spawn(move || {
        loop {
            // ownership of the receiver is moved to the thread
            let message = receiver2.recv();
            match message {
                Ok(message) => println!("Received from receiver2: {}", message),
                // RecvError implies that the sending half of the channel has been dropped
                Err(err) => {
                    println!("Error: {:?}", err); 
                    break;
                }
            }
        }
    });

    let broadcaster_thread = thread::spawn(move || {
            broadcaster.broadcast(vec!["Hello".to_string(), "World".to_string()]);
            broadcaster.broadcast(vec!["Random".to_string(), "Value".to_string()]);
    });

    // Keep the main thread alive
    // loop {}

    // Wait for the threads to finish
    receiver_thread_one.join().unwrap();
    receiver_thread_two.join().unwrap();
    broadcaster_thread.join().unwrap();
}

