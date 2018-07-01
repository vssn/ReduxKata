using System;

namespace ReduxKata
{
    class Store {
        private int state;
        private Func<int, Action, int> reducer;
        private Func<int, int> listener;

        public int getState() {
            return this.state;
        }
        public static Store createStore(Func<int, Action, int> reducer) {
            return new Store(reducer);
        }
        public int dispatch(Action action) {     
            this.state = this.reducer(this.state, action);
            listener(this.state);
            return this.state;
        }
        public void subscribe(Func <int, int> listener) {
            this.listener = listener;
        }
        
        Store(Func<int, Action, int> reducer) {
            this.reducer = reducer;
            this.state = 0;
        }
    }
}