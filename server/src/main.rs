use std::time::Duration;

mod asyncio;

async fn hello_async(i: u64) {
    asyncio::sleep(Duration::from_secs(5)).await;
    println!("Hello, async {}!", 5);
    asyncio::sleep(Duration::from_secs(5)).await;
    println!("Hello, async {}!", 10);
    //     if i < 10 {
    //         println!("Hello, async {}!", i);
    //         Box::pin(hello_async(i + 1)).await;
    //     } else if i < 20 {
    //         Box::pin(hello_async(i + 1)).await;
    //         asyncio::sleep(Duration::from_secs(25-i)).await;
    //         println!("Hello, async sleep {}!", i);
    //     }
}

fn main() {
    println!("Hello, world!");

    let ex = &mut asyncio::executor::Executor::new();
    ex.create_task(hello_async(0));
    ex.run();
}
