package com.company;

import java.util.List;
import java.util.Vector;
import java.util.function.BiFunction;
import java.util.function.Function;

public class Store{
    private int state;
    private BiFunction<Integer, Action, Integer> reducer;
    private Vector<Listener> listeners;

    private Store(BiFunction<Integer, Action, Integer> reducer) {
        this.reducer = reducer;
        this.listeners = new Vector<Listener>();
        this.state = 0;
    }

    static Store createStore(BiFunction<Integer, Action, Integer> reducer) {
        return new Store(reducer);
    }

    public void subscribe(Function<Integer, Integer> listener) {

        this.listeners.add(new Listener(listener));
    }

    public Integer dispatch(Action action) {
        this.state = this.reducer.apply(this.state, action);
        for(Listener listener: listeners) {
            listener.apply(this.state);
        }
        return this.state;
    }

    private static class Listener {
        private Function<Integer, Integer> listener;

        public Listener(Function<Integer, Integer> listener) {
            this.listener = listener;
        }

        public void apply(Integer state) {
            this.listener.apply(state);
        }
    }
}

