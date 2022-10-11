use crossbeam::channel::{self, Receiver, Sender};
use std::{thread, time::Duration};

#[derive(Debug)]
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    HotDog,
}

fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    for order in orders {
        println!("{} receives an order for {}", name, order);
        let lunch = match &order {
            x if x.contains("soup") => Lunch::Soup,
            x if x.contains("salad") => Lunch::Salad,
            x if x.contains("sandwich") => Lunch::Sandwich,
            _ => Lunch::HotDog,
        };
        for _ in 0..order.len() {
            thread::sleep(Duration::from_secs_f32(0.2))
        }
        println!("{} sends a {:?}", name, lunch);
        if lunches.send(lunch).is_err() {
            break;
        }
    }
}

fn main() {
    let (orders_tx, orders_rx) = channel::unbounded();
    let orders_rx2 = orders_rx.clone();
    let orders_rx3 = orders_rx.clone();

    let (lunches_tx, lunches_rx) = channel::unbounded();
    let lunches_tx2 = lunches_tx.clone();
    let lunches_tx3 = lunches_tx.clone();

    let zack_handle = thread::spawn(|| cafeteria_worker("zack", orders_rx2, lunches_tx2));
    let alice_handle = thread::spawn(|| cafeteria_worker("alice", orders_rx, lunches_tx));
    let mark_handle = thread::spawn(|| cafeteria_worker("mark", orders_rx3, lunches_tx3));

    for order in vec![
        "dog",
        "polish dog",
        "caesar salad",
        "onion soup",
        "reuben sandwich",
        "rosted babaganoush sandwich",
        "ei soup",
    ] {
        println!("ORDER: {}", order);
        let _ = orders_tx.send(order);
    }
    drop(orders_tx);

    for lunch in lunches_rx {
        println!("Order Up! -> {:?}", lunch);
    }

    let _ = alice_handle.join();
    let _ = zack_handle.join();
    let _ = mark_handle.join();
}
