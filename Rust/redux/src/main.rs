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
            let newState = state + 1;
            newState
        },
        Actions::DECREASE => {
            let newState = state - 1;
            newState
        },
        _ => {
            let newState = state;
            newState
        }
    }
}
