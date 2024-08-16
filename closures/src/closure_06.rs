use std::{sync::Arc, thread, time::Duration};

type Callback = dyn Fn(String) + Send + Sync + 'static;

fn call_callback<F>(f: F, message: String)
where
    F: Fn(String) + Send + Sync,
{
    f(message)
}

struct Runtime {
    callbacks: Vec<Arc<Callback>>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    fn register(&mut self, callback: Arc<Callback>) {
        self.callbacks.push(callback);
    }

    fn run(&self) {
        loop {
            thread::sleep(Duration::from_millis(500));

            self.callbacks
                .iter()
                .for_each(|cb| {
                    call_callback(cb.as_ref(), "Hello World".into())
            });
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    runtime.register(Arc::new(|message| {
        println!("{message}");
    }));

    runtime.run();
}
