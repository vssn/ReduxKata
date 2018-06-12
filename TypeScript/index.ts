interface Action {
    type: string
}

const counter = (state = 0, action: Action) => {
    switch (action.type) {
        case 'INCREMENT':
            return state += 1;
        case 'DECREMENT':
            return state -= 1
    }
    return state;
}

const createStore = (reducer: Function) => {
    var listeners = [];
    var state: number;
    var reducer: Function;
    return {
        dispatch: (action) => {
            state = reducer(state, action);
            listeners.map((listener) => {
                listener(state);
            })
        },
        subscribe: (listener) => {
            listeners.push(listener);
        },
        getState: () => {
            return state;
        }
    }
}

const log = (state) => {
    console.log(state);
}

const store = createStore(counter);
store.subscribe(log);

store.dispatch({ type: 'INCREMENT' })
store.dispatch({ type: 'INCREMENT' })
store.dispatch({ type: 'DECREMENT' })
