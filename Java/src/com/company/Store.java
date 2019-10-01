package com.company;

import java.util.function.BiFunction;
import java.util.function.Function;

public class Store{
    private int state;
    private BiFunction<Integer, Action, Integer> reducer;
    private Function<Integer, Integer> listener;

    private Store(BiFunction<Integer, Action, Integer> reducer) {
        this.reducer = reducer;
        this.state = 0;
    }

    public static Store createStore(BiFunction<Integer, Action, Integer> reducer) {
        return new Store(reducer);
    }

    public void subscribe(Function<Integer, Integer> listener) {
        this.listener = listener;
    }

    public Integer dispatch(Action action) {
        this.state = this.reducer.apply(this.state, action);
        listener.apply(this.state);
        return this.state;
    }
}
