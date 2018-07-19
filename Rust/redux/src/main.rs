extern crate redux;
use redux::*;

fn main() {
    let mut store = Store::create_store(0, &counter);
    store.subscribe(&log);

    store.dispatch(Actions::INCREASE);
    store.dispatch(Actions::INCREASE);
    store.dispatch(Actions::DECREASE);
}

fn log(state: u32) {
    println!("Counter: {}", state);
}

fn counter (state: u32, action: Actions) -> u32 {
    match action {
        Actions::INCREASE => {
            let state = state + 1;
            state
        },
        Actions::DECREASE => {
            let state = state - 1;
            state
        }
    }
}
