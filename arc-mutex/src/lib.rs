use core::assert;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::Send;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};

type ActorSet<T> = Arc<Mutex<HashSet<T>>>;
struct Actor<T: Send + Eq + Hash> {
    set: ActorSet<T>,
    rx: mpsc::Receiver<Command<T>>, // Must be mutable
    shutdown_rx: broadcast::Receiver<()>,
}

impl <T: Send + Eq + Hash> Actor<T> {
    fn new(set: ActorSet<T>,
           rx: mpsc::Receiver<Command<T>>,
           shutdown_rx: broadcast::Receiver<()>,
    ) -> Self {
        Actor {
            rx,
            set,
            shutdown_rx,
        }
    }

    async fn run(&mut self) {
        loop {
            tokio::select!{
                _ = self.shutdown_rx.recv() => return,
                Some(cmd) = self.rx.recv() => {
                    match cmd {
                        Command::Insert(x) => {
                            let _ = &self.set.lock().unwrap().insert(x);
                        },
                        Command::Remove(ref x) => {
                            let _ = &self.set.lock().unwrap().remove(x);
                        },
                    }
                },
            };
        }
    }
}

#[derive(Debug)]
enum Command<T> {
    Insert(T),
    Remove(T),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let (tx, rx) = mpsc::channel(16);
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let set = Arc::new(Mutex::new(HashSet::<u32>::new()));

        let mut actor = Actor::new(set.clone(), rx, shutdown_rx);

        let actor_handle = tokio::spawn(async move {
            actor.run().await;
        });

        // Insert 0..100
        for i in 0..100 {
            let _ = tx.send(Command::Insert(i));
        }

        // Remove even 0..100
        for i in 0..100 {
            if i % 2 == 0 {
                let _ = tx.send(Command::Remove(i));
            }
        }

        let _ = shutdown_tx.send(());
        let _ = tokio::join!(actor_handle);

        // Check that expected = actual.
        for i in 0..100 {
            let set_lock = set.lock().unwrap();

            assert_eq!(set_lock.len(), 50);

            if i % 2 != 0 {
                assert!(set_lock.contains(&i));
            }
        }
    }
}
