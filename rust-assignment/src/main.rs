use std::{thread, time::Duration};

use rand::{rng, seq::IndexedRandom, Rng};

struct User {
    id: usize,
    name: String,
    assigned_tickets: Vec<Ticket>,
    //extend to add more fields.
}

struct RequestTickets {
    user_id: usize,
    requested_tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]

struct Ticket {
    ticket_id: usize,
    seat: usize,
}

struct Receipt {
    user_id: usize,
    ticket_id: usize,
}

static NUMBER_OF_TICKETS: usize = 10;
static NUMBER_OF_USERS: usize = 1000;
static NUMBER_OF_REQUESTS: usize = 100000;

pub fn process_payment(user_id: usize) -> bool {
    let mut rng = rand::rng();
    thread::sleep(Duration::from_millis(rng.random_range(1..5))); // Simulate payment processing time
    rng.random_bool(0.7) // 70% chance of successful payment
}

fn main() {
    let mut rng = rng();

    let final_assigned_tickets: Vec<Receipt> = vec![];
    let all_tickets: Vec<Ticket> = (0..NUMBER_OF_TICKETS)
        .map(|i| Ticket {
            ticket_id: i,
            seat: i,
        })
        .collect();

    let requests: Vec<RequestTickets> = (0..NUMBER_OF_REQUESTS)
        .map(|_| {
            let user_id = rng.random_range(0..NUMBER_OF_USERS);
            let num_requested = rng.random_range(1..=3); // Each request can ask for 1-3 tickets
            let requested_tickets: Vec<Ticket> = all_tickets
                .choose_multiple(&mut rng, num_requested)
                .copied()
                .collect();

            RequestTickets {
                user_id,
                requested_tickets,
            }
        })
        .collect();

    let mut handles = vec![];

    for request in requests.into_iter() {
        let handle = thread::spawn(move || {
            println!(
                "🛒 User {} is attempting to book seats: {:?}",
                request.user_id,
                request
                    .requested_tickets
                    .iter()
                    .map(|t| t.seat)
                    .collect::<Vec<_>>()
            );

            // TODO: implement assignment logic
        });

        handles.push(handle);
    }
}
