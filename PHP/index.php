class Store {
    private $state;
    private $listeners = [];
    public $reducer;

    public function __construct($red) {
        $this->reducer = $red;
    }

    public static function createStore($reducer) {
        return new Store($reducer);
    }

    function getState() {
        return $this->state;
    }

    function subscribe($listener) {
        array_push($this->listeners, $listener);
    }
    
    function dispatch($action) {
        $reducer = $this->reducer;
        $this->state = $reducer($this->state, $action);

        foreach($this->listeners as $listener) {
            $listener($this);
        }
    }
}

$counter = function($state = 0, $action) {
    switch ($action) {
        case 'INCREASE':
            return $state += 1;
        case 'DECREASE':
            return $state += 1;
    }
    return $state;
};

$log = function($store) {
    print_r($store->getState() . "\n");
};

$store = Store::createStore($counter);
$store->subscribe($log);

$store->dispatch('INCREASE');
$store->dispatch('DECREASE');
$store->dispatch('INCREASE');

$store->getState();