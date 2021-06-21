const addon = require('./index.node');

function helloWorldExample() {
    const { hello } = addon;
    console.log('[HelloWorldExample] hello', hello);
}

function functionExample() {
    const { myFunc } = addon;
    const sumNumbers = myFunc(10, 10);
    console.log('[FunctionExample] myFunc(10, 10)', sumNumbers);
}

function promiseExample() {
    const { myAsyncFunc } = addon;
    myAsyncFunc(10)
        .then(result => {
            console.log('[PromiseExample] myAsyncFunc(10)', result);
        })
}

const map = {
    hello: helloWorldExample,
    func: functionExample,
    async: promiseExample
};


map[process.argv[2]]()
