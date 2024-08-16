use std::{sync::Arc, thread, time::Duration};

type Callback = dyn Fn(String) + Send + Sync + 'static;

struct Runtime {
    callback: Option<Arc<Callback>>,
}

impl Runtime {
    fn new() -> Self {
        Self { callback: None }
    }

    fn register(&mut self, callback: Arc<Callback>) {
        self.callback = Some(callback);
    }

    fn run(&self) {
        loop {
            thread::sleep(Duration::from_millis(500));

            let callback = match self.callback.clone() {
                Some(c) => c,
                None => panic!("No callback registered"),
            };

            (callback)("Hello World".into());
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
