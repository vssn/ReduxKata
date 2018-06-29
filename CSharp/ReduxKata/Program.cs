using System;

namespace ReduxKata
{
    class Program
    {
        static void Main(string[] args)
        {
            Func<int, string, int> counter = delegate(int state, string action) {
                return state;
            };
            
            var store = Store.createStore(counter);
            Console.Write(store.getState());
        }

        

    }

    class Store {
        private int state;
        private Func<int, string, int> reducer;
        public int getState() {
            return this.state;
        }
        public static Store createStore(Func<int, string, int> reducer) {
            return new Store(reducer);
        }

        Store(Func<int, string, int> reducer) {
            this.reducer = reducer;
            this.state = 0;
        }


    }

    class Reducer {

    }
}
