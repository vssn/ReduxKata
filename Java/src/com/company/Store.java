package com.company;

import java.util.Vector;
import java.util.function.BiFunction;
import java.util.function.Function;

public class Store{
    private int state;
    private BiFunction<Integer, Action, Integer> reducer;
    private Vector<Function<Integer, Integer>> listeners;

    private Store(BiFunction<Integer, Action, Integer> reducer) {
        this.reducer = reducer;
        this.listeners = new Vector<Function<Integer, Integer>>();
        this.state = 0;
    }

    public static Store createStore(BiFunction<Integer, Action, Integer> reducer) {
        return new Store(reducer);
    }

    public void subscribe(Function<Integer, Integer> listener) {
        this.listeners.add(listener);
    }

    public Integer dispatch(Action action) {
        this.state = this.reducer.apply(this.state, action);
        for(Function<Integer, Integer> listener: listeners) {
            listener.apply(this.state);
        }
        return this.state;
    }
}
