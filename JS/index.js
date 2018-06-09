const expect = require('expect');

/**
 * REDUX FACTORY usually via Library available
 * @param {Function} reducer use case specific state and action handler
 * @return {Object} methods to get current state, send/dispatch and listen/subscribe
 */
const createStore = (reducer) => {
  let state;
  let listeners = [];
  
  const getState = () => state;
  
  const dispatch = (action) => {
    state = reducer(state, action);
    listeners.forEach(listener => listener());
  };
  
  const subscribe = (listener) => {
    listeners.push(listener);
    return () => {
      listeners = listeners.filter(l => l !== listener);
    };
  };
  
  dispatch({});
  
  return { getState, dispatch, subscribe };
};

/**
 *	REDUCER specifically for counter use case
 * @param {Object} state with application state
 * @param {String} action keyword
 * @return {Object} state which was either changed or not
 */
const counter = (state = 0, action) => {
  switch (action.type) {
    case 'INCREMENT':
      return state + 1;
    case 'DECREMENT':
      return state - 1;
    default: 
      return state;
  }
}

/**
 *	Application store
 */ 
const store = createStore(counter);

/**
 *	Use case counter applying to environment specific view
 */
const render = () => {
  console.log(store.getState())
};

/**
 *	Use case counter applying to view
 */
store.subscribe(render);

store.dispatch({ type: 'INCREMENT' });
expect(store.getState()).toBe(1);

store.dispatch({ type: 'INCREMENT' });
expect(store.getState()).toBe(2);

store.dispatch({ type: 'DECREMENT' });
expect(store.getState()).toBe(1);

console.log('tests passing');

