use std::io::timer::sleep;

struct Philosopher {
    name: String,
    sender: Sender<int>,
    receiver: Receiver<bool>,
    left_hand: uint,
    right_hand: uint,
}

impl Philosopher {
    fn eat(&self) {
        println!("{} has sat down to eat.", self.name);

        for _ in range(1i, 3) {
            println!("{} is thinking.", self.name);

            sleep(10_000u64);

            println!("{} is hungry.", self.name);

            loop {
                self.sender.send(self.left_hand as int);
                if self.receiver.recv() { break; }
            }

            println!("{} picked up their left chopstick.", self.name);

            loop {
                self.sender.send(self.right_hand as int);
                if self.receiver.recv() { break; }
            }

            println!("{} picked up their right chopstick.", self.name);
            println!("{} is eating.", self.name);
            
            sleep(10_000u64);

            println!("{} is done eating.", self.name);

            self.sender.send(self.left_hand as int * -1);
            println!("{} has put down their left chopstick.", self.name);
            self.sender.send(self.right_hand as int * -1);
            println!("{} has put down their right chopstick.", self.name);
        }

        self.sender.send(0);
        self.receiver.recv();

        println!("{} is done with their meal.", self.name);
    }

    fn new(name: &str,
           left_hand: uint,
           right_hand: uint) -> (Philosopher, Sender<bool>, Receiver<int>) {
        let (tx, rx)   = channel();
        let (tx1, rx1) = channel();

        let p = Philosopher {
            name: name.to_string(),
            sender: tx,
            receiver: rx1,
            left_hand: left_hand,
            right_hand: right_hand,
        };
        
        (p, tx1, rx)
    }
}

fn main() {
    let (p, tx1, rx1)  = Philosopher::new("Karl Marx", 1, 2);
    spawn(proc() { p.eat() });

    let (p, tx2, rx2) = Philosopher::new("Gilles Deleuze", 2, 3);
    spawn(proc() { p.eat() });

    let (p, tx3, rx3) = Philosopher::new("Baruch Spinoza", 3, 4);
    spawn(proc() { p.eat() });

    let (p, tx4, rx4) = Philosopher::new("Friedrich Nietzsche", 4, 5);
    spawn(proc() { p.eat() });

    // Foucault is left handed. ;)
    let (p, tx5, rx5) = Philosopher::new("Michel Foucault", 1, 5);
    spawn(proc() { p.eat() });

    let mut chopsticks = [false, false, false, false, false];

    let mut remaining = 5u;
    while remaining != 0 {
        process_philosopher(&mut chopsticks, &tx1, &rx1, &mut remaining);
        process_philosopher(&mut chopsticks, &tx2, &rx2, &mut remaining);
        process_philosopher(&mut chopsticks, &tx3, &rx3, &mut remaining);
        process_philosopher(&mut chopsticks, &tx4, &rx4, &mut remaining);
        process_philosopher(&mut chopsticks, &tx5, &rx5, &mut remaining);
        std::task::deschedule();
    }

    println!("Done!");
}

fn process_philosopher(chopsticks: &mut [bool, ..5],
                       tx: &Sender<bool>,
                       rx: &Receiver<int>,
                       remaining: &mut uint) {

    let response = match rx.try_recv() {
        Ok(i) => i,
        Err(_) => return,
    };
        
    match response {
        0 => {
            *remaining += -1;
            tx.send(false);
        },
        x if x > 0 => {
            if chopsticks[(x - 1) as uint] {
                tx.send(false);
            } else {
                chopsticks[(x - 1) as uint] = true;
                tx.send(true);
            }
        },
        x => { chopsticks[((-x) - 1) as uint] = false; },
    }
}
