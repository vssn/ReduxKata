import Foundation

enum Action {
    case increase
    case decrease
}

class Store {
    var state: Int
    var listeners:[(Store)->()] = []
    var reducer: (Int, Action)->Int
    
    init(_ reducer: @escaping (Int, Action)->Int) {
        self.state = 0
        self.reducer = reducer
    }
    
    func dispatch(action: Action) -> Store {
        state = self.reducer(state, action)
        listeners.forEach { listener in
            listener(self)
        }
        return self
    }
    
    func subscribe(method: @escaping (Store)->()) {
        listeners.append(method)
    }
    
    func getState() -> Int {
        return state
    }
}

class Redux {
    func createStore(reducer: @escaping (Int, Action) -> Int) -> Store {
        let store = Store(reducer)
        return store
    }
    
    func counter(state: Int, action: Action) -> Int {
        switch action {
        case .increase:
            return state + 1
        case .decrease:
            return state - 1
        }
        
        return state
    }
}

class Main {
    let store: Store
    
    init() {
        let redux = Redux()
        store = redux.createStore(reducer: redux.counter)
        
        store.subscribe(method: self.render)

        
        store.dispatch(action: .increase)
        store.dispatch(action: .increase)
        store.dispatch(action: .decrease)
    }
    
    func render(store: Store) {
        print("Render method: ", store.getState())
    }
}

Main()
