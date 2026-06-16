// Use crossbeam_channel instead of std::sync::mpsc
use crossbeam_channel::{bounded, SendError}; 
use std::{thread, time::Duration};

fn main() {
    // 1. Create a bounded MPMC channel
    let (tx1, rx1) = bounded(10);
    let tx2 = tx1.clone();
    let rx2 = rx1.clone(); // Multi-Consumer: We can clone the receiver!

    // --- PUBLISHERS (Same as before) ---
    let producer_handle1 = thread::spawn(move || {
        let mut i = 0;
        loop {
            thread::sleep(Duration::from_millis(10));
            i += 1;
            if let Err(SendError(_)) = tx1.send(format!("Publisher-1 >> {i}")) {
                println!("Publisher 1 stopped.");
                break;
            }
        }
    });

    let producer_handle2 = thread::spawn(move || {
        let mut i = 0;
        loop {
            thread::sleep(Duration::from_millis(10));
            i += 1;
            if let Err(SendError(_)) = tx2.send(format!("Publisher-2 >> {i}")) {
                println!("Publisher 2 stopped.");
                break;
            }
        }
    });

    // --- MULTIPLE RECEIVERS ---
    // Receiver Thread 1
    let receiver_handle1 = thread::spawn(move || {
        while let Ok(msg) = rx1.recv() {
            thread::sleep(Duration::from_millis(15)); // Simulating work
            println!("Receiver [A] processed: {msg}");
        }
        println!("Receiver [A] stopped.");
    });

    // Receiver Thread 2
    let receiver_handle2 = thread::spawn(move || {
        while let Ok(msg) = rx2.recv() {
            thread::sleep(Duration::from_millis(15)); // Simulating work
            println!("Receiver [B] processed: {msg}");
        }
        println!("Receiver [B] stopped.");
    });

    // Wait for all threads
    let _ = producer_handle1.join();
    let _ = producer_handle2.join();
    let _ = receiver_handle1.join();
    let _ = receiver_handle2.join();
}
