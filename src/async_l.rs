use tokio::prelude::*;
use tokio::await;
use std::marker::Unpin;

async fn sleep(n: u64) {
    use std::time::{Duration, Instant};
    use tokio::timer::Delay;
    await!(Delay::new(Instant::now() + Duration::from_secs(n))).unwrap();
}

// sleep a second before each line is printed:
pub fn al_print_sleep(){
    tokio::run_async(async {
        await!(sleep(1));
        println!("One");
        await!(sleep(1));
        println!("Two");
        await!(sleep(1));
        println!("Three");
    });

}

pub fn transform_o2n(){
    use std::future::Future as NewFuture;
    use futures::Future as OldFuture;

    // converts from an old style Future to a new style one:
    fn forward<I,E>(f: impl OldFuture<Item=I, Error=E> + Unpin) -> impl NewFuture<Output=Result<I,E>> {
        use tokio_async_await::compat::forward::IntoAwaitable;
        f.into_awaitable()
    }
    tokio::run_async(async {
        // Create some old style Future:
        let old_future = futures::future::ok::<_,()>("Awaiting a manually converted 0.1 future!");
        // Convert to a new style one:
        let new_future = forward(old_future);
        // `await` the result and print it:
        println!("{}", std::await!(new_future).unwrap());
    });
}

async fn hello_world() -> &'static str {
    "Hello World"
}

pub fn transform_n2o(){
    use std::future::Future as NewFuture;
    use futures::Future as OldFuture;


    // converts from a new style Future to an old style one:
    fn backward<I,E>(f: impl NewFuture<Output=Result<I,E>>) -> impl OldFuture<Item=I, Error=E> {
        use tokio_async_await::compat::backward;
        backward::Compat::new(f)
    };
// Map our hello_world() future to return a Result<&str,()> rather
// than just &'str, so that we can convert it to an old style one:
    let hello_world_result = async {
        let s = await!(hello_world());
        Ok::<_,()>(s)
    };

// use the above function to convert back:
    let hello_world_old = backward(hello_world_result);

// We can then run it like any old style future, allowing to to use any
// of the machinery currently available for 0.1 style futures:
    tokio::run(
        {
            hello_world_old.map(|val| println!("Running as 0.1 future: {}", val))
        }
    );
}

pub fn execution_concurrently(){
    tokio::run_async(async {
        async fn write_to_stdout() {
            let mut stdout = tokio::io::stdout();
            let message = "Concurrently Writing This Message";

            for byte in message.bytes() {
                let buf = &[byte];
                await!(stdout.write_all_async(buf)).unwrap();
                await!(stdout.flush_async()).unwrap();
            }
        }

        // execute this future-returning fn multiple times
        // concurrently:
        tokio::spawn_async(write_to_stdout());
        tokio::spawn_async(write_to_stdout());
        tokio::spawn_async(write_to_stdout());
        tokio::spawn_async(write_to_stdout());
        tokio::spawn_async(write_to_stdout());
        tokio::spawn_async(write_to_stdout());

    });
}