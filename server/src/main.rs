mod async_runtime;

async fn hello_async(i: i32) {
    if i < 10 {
        println!("Hello, async {}!", i);
        Box::pin(hello_async(i + 1)).await;
    }
}

fn main() {
    println!("Hello, world!");

    let ex = &mut async_runtime::executor::Executor::new();
    ex.create_task(hello_async(0));
    ex.run();
}
