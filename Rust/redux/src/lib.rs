use std::cell::Cell;

pub enum Actions {
    INCREASE,
    DECREASE
}

pub struct Store {
    state: Cell<u32>,
    listeners: Vec<&'static Fn(u32)>,
    reducer: &'static Fn(u32, Actions) -> u32
}

impl Store {
    pub fn create_store(state: u32, reducer: &'static Fn(u32, Actions) -> u32 ) -> Store {
        Store { state: Cell::new(state), listeners: Vec::new(), reducer }
    }
    pub fn get_state(&self) -> u32 {
        self.state.get()
    }
    pub fn subscribe(&mut self, listener: &'static Fn(u32)) {
        self.listeners.push(listener);
    }
    pub fn dispatch(&mut self, action: Actions) {
        self.state = Cell::new((*self.reducer)(self.state.get(), action));
        for listener in self.listeners.iter() {
            listener(self.state.get());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Sample Reducer
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

    #[test]
    fn it_subscribes_a_reducer() {
        let store = Store::create_store(0, &counter);
        assert_eq!(store.get_state(), 0);
    }

    #[test]
    fn it_dispatches_an_action() {
        let c = &counter;
        let mut store = Store::create_store(0, c);

        store.dispatch(Actions.INCREASE);

        assert_eq!(store.get_state(), 1);
    }
}