use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

type ActorSet<T> = Arc<Mutex<HashSet<T>>>;
struct Actor<T> {
    rx: mpsc::Receiver<Command<T>>,
    map: ActorSet<T>,
}

impl <T> Actor<T> {
    fn new(set: ActorSet<T>, rx: mpsc::Receiver<T>) -> Self {
        todo!()
    }

    async fn run() -> Result<(),()> {
        todo!()
    }
}

enum Command<T> {
    Insert(T),
    Delete(T),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
