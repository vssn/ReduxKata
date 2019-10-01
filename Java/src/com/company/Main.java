package com.company;
import java.util.function.BiFunction;
import java.util.function.Function;

public class Main {
    public static void main(String... args) {
        BiFunction<Integer, Action, Integer> counter = (Integer state, Action action) -> {
            switch (action) {
                case INCREASE:
                    return state += 1;
                case DECREASE:
                    return state -= 1;
                default:
                    return state;
            }
        };

        Function<Integer, Integer> render = (Integer state) -> {
            System.out.println(state);
            return state;
        };

        Store store = Store.createStore(counter);
        store.subscribe(render);
        store.dispatch(Action.INCREASE);
        store.dispatch(Action.DECREASE);
        store.dispatch(Action.INCREASE);
    }
}

