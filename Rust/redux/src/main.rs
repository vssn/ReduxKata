extern crate redux;
use redux::*;

fn main() {
    let mut store = Store::create_store(0, &counter);
    store.subscribe(&log);

    store.dispatch(Action::INCREASE);
    store.dispatch(Action::INCREASE);
    store.dispatch(Action::DECREASE);
}

fn log(state: u32) {
    println!("Counter: {}", state);
}

fn counter (state: u32, action: Action) -> u32 {
    match action {
        Action::INCREASE => state + 1,
        Action::DECREASE => state - 1
    }
}
