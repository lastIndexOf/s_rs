use std::time::Duration;

// current thread
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);

    for i in 0..10 {
        handles.push(rt.spawn(async move {
            let time = 1000 - 50 * i;
            println!("thread {i} is start, time = {time}");
            tokio::time::sleep(Duration::from_millis(time)).await;
            println!("thread {i} is done");
        }));
    }

    std::thread::sleep(Duration::from_millis(750));
    println!("in main thread sleep 750 done");

    for (i, handle) in handles.into_iter().enumerate() {
        println!("{i} handle is running");
        rt.block_on(handle).unwrap();
    }
}

// multi thread
// fn main() {
//     let rt = tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap();

//     let mut handles = Vec::with_capacity(10);

//     for i in 0..10 {
//         handles.push(rt.spawn(async move {
//             let time = 1000 - 50 * i;
//             println!("thread {i} is start, time = {time}");
//             tokio::time::sleep(Duration::from_millis(time)).await;
//             println!("thread {i} is done");
//         }));
//     }

//     std::thread::sleep(Duration::from_millis(750));
//     println!("in main thread sleep 750 done");

//     for handle in handles {
//         rt.block_on(handle).unwrap();
//     }
// }
