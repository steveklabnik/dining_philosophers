use std::io::timer::sleep;

enum PickupPermission {
    Allowed,
    NotAllowed, 
}

struct Philosopher {
    name: String,
    sender: Sender<int>,
    receiver: Receiver<PickupPermission>,
    first_chopstick: uint,
    second_chopstick: uint,
}

impl Philosopher {
    fn eat(&self) {
        println!("{} has sat down to eat.", self.name);

        for _ in range(1i, 3) {
            println!("{} is thinking.", self.name);

            sleep(10_000u64);

            println!("{} is hungry.", self.name);

            loop {
                self.sender.send(self.first_chopstick as int);
                match self.receiver.recv() { Allowed => break, _ => {}}
            }

            println!("{} picked up their first chopstick.", self.name);

            loop {
                self.sender.send(self.second_chopstick as int);
                match self.receiver.recv() { Allowed => break, _ => {}}
            }

            println!("{} picked up their second chopstick.", self.name);
            println!("{} is eating.", self.name);
            
            sleep(10_000u64);

            println!("{} is done eating.", self.name);

            self.sender.send(self.first_chopstick as int * -1);
            println!("{} has put down their first chopstick.", self.name);
            self.sender.send(self.second_chopstick as int * -1);
            println!("{} has put down their second chopstick.", self.name);
        }

        self.sender.send(0);
        self.receiver.recv();

        println!("{} is done with their meal.", self.name);
    }

    fn new(name: &str,
           first_chopstick: uint,
           second_chopstick: uint) -> (Philosopher, Sender<PickupPermission>, Receiver<int>) {
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
                       tx: &Sender<PickupPermission>,
                       rx: &Receiver<int>,
                       remaining: &mut uint) {

    let response = match rx.try_recv() {
        Ok(i) => i,
        Err(_) => return,
    };
        
    match response {
        0 => {
            *remaining += -1;
            tx.send(NotAllowed);
        },
        x if x > 0 => {
            if chopsticks[(x - 1) as uint] {
                tx.send(NotAllowed);
            } else {
                chopsticks[(x - 1) as uint] = true;
                tx.send(Allowed);
            }
        },
        x => { chopsticks[((-x) - 1) as uint] = false; },
    }
}
