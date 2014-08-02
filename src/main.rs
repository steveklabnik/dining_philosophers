struct Philosopher {
    name: String,
    channel: Sender<int>,
    left_hand: int,
    right_hand: int,
}

impl Philosopher {
    fn eat(&self) {
        println!("{} is eating.", self.name);
    }

    fn new(name: &str,
           left_hand: int,
           right_hand: int) -> (Philosopher, Receiver<int>) {
        let (tx, rx) = channel();
        let p = Philosopher {
            name: name.to_string(),
            channel: tx,
            left_hand: 1,
            right_hand: 2,
        };
        
        (p, rx)
    }
}

enum Status {
    OnTable,
    InUse,
}

struct Chopstick(Status);

fn main() {
    let (p, rx) = Philosopher::new("Karl Marx", 1, 2,);

    spawn(proc() {
        p.eat();
    });

    let (p, rx) = Philosopher::new("Gilles Deleuze", 2, 3);

    spawn(proc() {
        p.eat();
    });

    let (p, rx) = Philosopher::new("Baruch Spinoza", 3, 4);

    spawn(proc() {
        p.eat();
    });

    let (p, rx) = Philosopher::new("Friedrich Nietzsche", 4, 5);

    spawn(proc() {
        p.eat();
    });

    // Foucault is left handed. ;)
    let (p, rx) = Philosopher::new("Michel Foucault", 1, 5);

    spawn(proc() {
        p.eat();
    });

    let mut chopsticks = Vec::from_fn(5, |_| Chopstick(OnTable));
}

