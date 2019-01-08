use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

struct Philosopher {
    name: String,
    done: Sender<bool>,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, done: Sender<bool>, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            done: done,
            left: left,
            right: right,
        }
    }

    fn done(&self) {
        println!("{} is done eating.", self.name);

        self.done.send(true).ok().expect("Couldn't finish eating");
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left]
            .lock()
            .ok()
            .expect("Couldn't aquire left mutex");
        let _right = table.forks[self.right]
            .lock()
            .ok()
            .expect("Couldn't aquire right mutex");

        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        self.done();
    }
}

struct Table {
    forks: Vec<Mutex<bool>>,
}

fn main() {
    let (done_tx, done_rx) = mpsc::channel();

    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", done_tx.clone(), 0, 1),
        Philosopher::new("Gilles Deleuze", done_tx.clone(), 1, 2),
        Philosopher::new("Karl Marx", done_tx.clone(), 2, 3),
        Philosopher::new("Friedrich Nietzsche", done_tx.clone(), 3, 4),
        Philosopher::new("Michel Foucault", done_tx.clone(), 0, 4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for _ in 0..5 {
        done_rx.recv().unwrap();
    }

    for h in handles {
        h.join().ok().expect("Couldn't join a thread.");
    }
}
