use std::io::timer::sleep;

struct Philosopher {
    name: String,
    sender: Sender<PhilosopherAction>,
    receiver: Receiver<PickupPermission>,
    first_chopstick: uint,
    second_chopstick: uint,
}

enum PhilosopherAction {
    Take(uint),
    Put(uint),
    Sated,
}

enum PickupPermission {
    Allowed,
    NotAllowed, 
}

impl Philosopher {
    fn eat(&self) {
        println!("{} has sat down to eat.", self.name);

        for _ in range(1i, 3) {
            println!("{} is thinking.", self.name);
            sleep(10_000u64);
            println!("{} is hungry.", self.name);

            self.take_first_chopstick();
            self.take_second_chopstick();

            println!("{} is eating.", self.name);
            sleep(10_000u64);
            println!("{} is done eating.", self.name);

            self.put_first_chopstick();
            self.put_second_chopstick();
        }

        self.sender.send(Sated);

        println!("{} is done with their meal.", self.name);
    }

    fn take_first_chopstick(&self) {
        self.take_chopstick(self.first_chopstick);
        println!("{} picked up their first chopstick.", self.name);
    }

    fn take_second_chopstick(&self) {
        self.take_chopstick(self.second_chopstick);
        println!("{} picked up their second chopstick.", self.name);
    }

    fn take_chopstick(&self, chopstick: uint) {
        loop {
            self.sender.send(Take(chopstick));
            match self.receiver.recv() { Allowed => break, _ => {}}
        }
    }

    fn put_first_chopstick(&self) {
        self.put_chopstick(self.first_chopstick);
        println!("{} has put down their first chopstick.", self.name);
    }

    fn put_second_chopstick(&self) {
        self.put_chopstick(self.second_chopstick);
        println!("{} has put down their second chopstick.", self.name);
    }

    fn put_chopstick(&self, chopstick: uint) {
        self.sender.send(Put(chopstick));
    }

    fn new(name: &str,
           first_chopstick: uint,
           second_chopstick: uint) -> (Philosopher, Sender<PickupPermission>, Receiver<PhilosopherAction>) {
        let (tx, rx)   = channel();
        let (tx1, rx1) = channel();

        let p = Philosopher {
            name: name.to_string(),
            sender: tx,
            receiver: rx1,
            first_chopstick: first_chopstick,
            second_chopstick: second_chopstick,
        };
        
        (p, tx1, rx)
    }
}

struct Table {
    remaining: int,
    chopsticks: [bool, ..5],
    philosophers: [(Sender<PickupPermission>,
                    Receiver<PhilosopherAction>), ..5],
}

impl Table {
    fn new(philosophers: [(Sender<PickupPermission>,
                           Receiver<PhilosopherAction>), ..5]) -> Table {
        Table {
            remaining: 5i,
            chopsticks: [false, false, false, false, false],
            philosophers: philosophers,
        }
    }

    fn have_dinner(&mut self) {
        while self.remaining != 0 {
            for &(ref tx, ref rx) in self.philosophers.iter() {
                let response = match rx.try_recv() {
                    Ok(action) => action,
                    Err(_) => continue,
                };

                match response {
                    Sated => { self.remaining += -1 },
                    Take(x) => {
                        if self.chopsticks[x - 1] {
                            tx.send(NotAllowed);
                        } else {
                            self.chopsticks[x - 1] = true;
                            tx.send(Allowed);
                        }
                    },
                    Put(x) => { self.chopsticks[x - 1] = false; },
                }
            }
        }
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

    let mut table = Table::new([
        (tx1, rx1),
        (tx2, rx2),
        (tx3, rx3),
        (tx4, rx4),
        (tx5, rx5),
    ]);

    table.have_dinner();

    println!("Done!");
}
