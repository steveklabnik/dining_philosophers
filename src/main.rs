#![feature(std_misc)]
#![feature(core)]
#![feature(io)]

use std::old_io::timer::sleep;
use std::time::Duration;
use std::thread::Thread;
use std::sync::mpsc;

struct Philosopher {
    name: String,
    channel: PhilosopherChannel<PhilosopherAction, PickupPermission>,
    first_chopstick: u32,
    second_chopstick: u32,
}

enum PhilosopherAction {
    Take(u32),
    Put(u32),
    Sated,
}

enum PickupPermission {
    Allowed,
    NotAllowed,
}

struct PhilosopherChannel<S, R> {
    tx: mpsc::Sender<S>,
    rx: mpsc::Receiver<R>,
}

fn make_channel() -> (PhilosopherChannel<PhilosopherAction, PickupPermission>,
                    PhilosopherChannel<PickupPermission, PhilosopherAction>) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    (PhilosopherChannel { tx: tx1, rx: rx2 },
     PhilosopherChannel { tx: tx2, rx: rx1 })
}

impl<S:Send,R:Send> PhilosopherChannel<S, R> {
    fn send(&self, x: S) {
        self.tx.send(x).unwrap()
    }
    fn recv(&self) -> R {
        self.rx.recv().unwrap()
    }
    fn try_recv(&self) -> Result<R, mpsc::TryRecvError> {
        self.rx.try_recv()
    }
}

impl Philosopher {
    fn eat(&self) {
        println!("{} has sat down to eat.", self.name);

        for _ in range(1, 3) {
            println!("{} is thinking.", self.name);
            sleep(Duration::microseconds(10));
            println!("{} is hungry.", self.name);

            self.take_first_chopstick();
            self.take_second_chopstick();

            println!("{} is eating.", self.name);
            sleep(Duration::microseconds(10));
            println!("{} is done eating.", self.name);

            self.put_first_chopstick();
            self.put_second_chopstick();
        }

        self.channel.send(PhilosopherAction::Sated);

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

    fn take_chopstick(&self, chopstick: u32) {
        loop {
            self.channel.send(PhilosopherAction::Take(chopstick));
            match self.channel.recv() { PickupPermission::Allowed => break, _ => {}}
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

    fn put_chopstick(&self, chopstick: u32) {
        self.channel.send(PhilosopherAction::Put(chopstick));
    }

    fn new(name: &str,
           first_chopstick: u32,
           second_chopstick: u32) -> (Philosopher,
                                       PhilosopherChannel<PickupPermission,
                                                          PhilosopherAction>) {
        let (c1, c2) = make_channel();

        let p = Philosopher {
            name: name.to_string(),
            channel: c1,
            first_chopstick: first_chopstick,
            second_chopstick: second_chopstick,
        };

        (p, c2)
    }
}

struct Table {
    remaining: i32,
    chopsticks: [bool; 5],
    philosophers: [PhilosopherChannel<PickupPermission, PhilosopherAction>; 5],
}

impl Table {
    fn new(philosophers: [PhilosopherChannel<PickupPermission,
                                             PhilosopherAction>; 5]) -> Table {
        Table {
            remaining: 5,
            chopsticks: [false, false, false, false, false],
            philosophers: philosophers,
        }
    }

    fn have_dinner(&mut self) {
        while self.remaining != 0 {
            for channel in self.philosophers.iter() {
                let response = match channel.try_recv() {
                    Ok(action) => action,
                    Err(_) => continue,
                };

                match response {
                    PhilosopherAction::Sated => { self.remaining += -1 },
                    PhilosopherAction::Take(x) => {
                        if self.chopsticks[(x - 1) as usize] {
                            channel.send(PickupPermission::NotAllowed);
                        } else {
                            self.chopsticks[(x - 1) as usize] = true;
                            channel.send(PickupPermission::Allowed);
                        }
                    },
                    PhilosopherAction::Put(x) => { self.chopsticks[(x - 1) as usize] = false; },
                }
            }
        }
    }
}

fn main() {
    let (p, c1)  = Philosopher::new("Karl Marx", 1, 2);
    let _ = Thread::spawn(move || { p.eat() });

    let (p, c2) = Philosopher::new("Gilles Deleuze", 2, 3);
    let _ = Thread::spawn(move || { p.eat() });

    let (p, c3) = Philosopher::new("Baruch Spinoza", 3, 4);
    let _ = Thread::spawn(move || { p.eat() });

    let (p, c4) = Philosopher::new("Friedrich Nietzsche", 4, 5);
    let _ = Thread::spawn(move || { p.eat() });

    // Foucault is left handed. ;)
    let (p, c5) = Philosopher::new("Michel Foucault", 1, 5);
    let _ = Thread::spawn(move || { p.eat() });

    let mut table = Table::new([c1, c2, c3, c4, c5]);

    table.have_dinner();

    println!("Done!");
}
