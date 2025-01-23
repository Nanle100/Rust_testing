// // Asynchronous programming is a way to run multiple tasks concurrently without blocking the execution of other tasks. 
// // In Rust, this is achieved using async and await keywords, along with an async runtime like Tokio.

// // 1. async: Marks a function or block of code as asynchronous, meaning it can pause its execution to wait for something (e.g., an I/O operation) and let other tasks run in the meantime.

// // 2. await:  Used inside async functions to pause the execution of that function until the awaited operation is complete.



// // +++ async Functions: +++

// // When you declare a function with async, it does not execute immediately. Instead, it returns a future.
// // A future is like a "promise" to do some work later. The actual work only starts when the future is awaited.


// // async fn example() {
// //     println!("This will run when the function is awaited.");
// // }

// // #[tokio::main]
// // async fn main() {
// //     let future = example(); // The async function doesn't run yet.
// //     future.await; // Now it runs!
// // }


// // +++ await Functions: +++
// //The await keyword pauses the current async function until the awaited future completes.
// //While the current function is paused, other tasks in the runtime can run.


// use tokio::time::{sleep, Duration};

// async fn task_one() {
//     println!("Task one started...");
//     sleep(Duration::from_secs(2)).await; // Pause here for 2 seconds
//     println!("Task one completed!");
// }

// async fn task_two() {
//     println!("Task two started...");
//     sleep(Duration::from_secs(1)).await; // Pause here for 1 second
//     println!("Task two completed!");
// }

// // async fn main() {
// //     task_one().await; // Wait for task_one to finish
// //     task_two().await; // Wait for task_two to finish
// // }

// // note : task_one() is paused during the sleep for 2 seconds. After task_one() finishes, task_two() starts.

// //we can make it work concurrently


// #[tokio::main]
// async fn main(){

//     tokio::join!(task_one(), task_two());
// }



// //NOTE:
// // An async function does not run until you .await it or include it in a concurrent executor (like tokio::join!).
// // await pauses the current function and allows other async tasks to run in the background.
// // You can control whether tasks run sequentially (using await one after the other) or concurrently (using tokio::join!).


// async fn fetch_data() -> String {
//     "Fetched data!".to_string()
// }

// async fn process_data(data: String) -> String {
//     format!("Processed: {}", data)
// }

// #[tokio::main]
// async fn main() {
//     let data = fetch_data().await;
//     let processed = process_data(data.clone()).await;
//     println!("{}", processed);
//     print!("{}", data);
// }

use tokio::time::{sleep, Duration};

async fn task_one() {
    sleep(Duration::from_secs(2)).await;
    println!("Task One Done!");
}

async fn task_two() {
    sleep(Duration::from_secs(1)).await;
    println!("Task Two Done!");
}

#[tokio::main]
async fn main() {
    tokio::join!(task_one(), task_two()); // Run tasks concurrently
    println!("Both tasks are done!");
}

