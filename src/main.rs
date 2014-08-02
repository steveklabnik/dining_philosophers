struct Philosopher {
    name: String,
    channel: Sender<int>,
    left_hand: int,
    right_hand: int,
}

enum Status {
    OnTable,
    InUse,
}

struct Chopstick(Status);

fn main() {
    let channels = Vec::from_fn(4, |_| channel::<int>());

    let philosophers = vec![
        Philosopher {
            name: "Karl Marx".to_string(),
            channel: channels[0].ref0().clone(),
            left_hand: 1,
            right_hand: 2,
        },
        Philosopher {
            name: "Gilles Deleuze".to_string(),
            channel: channels[1].ref0().clone(),
            left_hand: 2,
            right_hand: 3,
        },
        Philosopher {
            name: "Baruch Spinoza".to_string(),
            channel: channels[0].ref0().clone(),
            left_hand: 3,
            right_hand: 4,
        },
        Philosopher {
            name: "Friedrich Nietzsche".to_string(),
            channel: channels[0].ref0().clone(),
            left_hand: 4,
            right_hand: 5,
        },
        Philosopher {
            name: "Michel Foucault".to_string(),
            channel: channels[0].ref0().clone(),
            // Foucault is left handed. ;)
            left_hand: 1,
            right_hand: 5,
        },
    ];

    let mut chopsticks = Vec::from_fn(5, |_| Chopstick(OnTable));
}

