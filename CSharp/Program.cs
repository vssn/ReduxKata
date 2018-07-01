using System;

namespace ReduxKata
{
    class Program
    {
        static void Main(string[] args)
        {
            Func<int, Action, int> counter = delegate(int state, Action action) {
                switch(action) {
                    case Action.INCREASE:
                        return state += 1;
                    case Action.DECREASE:
                        return state += 1;
                }
                return state;
            };
            
            Func<int, int> render = delegate(int state) {
                Console.Write(state);
                Console.Write('\n');
                return state;
            };
            
            var store = Store.createStore(counter);
            store.subscribe(render);
            store.dispatch(Action.INCREASE);
            store.dispatch(Action.DECREASE);
            store.dispatch(Action.INCREASE);
        }
    }
}
