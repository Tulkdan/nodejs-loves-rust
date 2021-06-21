const addon = require('./index.node');

function helloWorldExample() {
    const { hello } = addon;
    console.log('[HelloWorldExample] hello ->', hello());
}

function functionExample() {
    const { add } = addon;
    const sumNumbers = add(10, 10);
    console.log('[FunctionExample] add(10, 10) ->', sumNumbers);
}

function promiseExample() {
    const { myAsyncFunc } = addon;
    myAsyncFunc(10)
        .then(result => {
            console.log('[PromiseExample] myAsyncFunc(10) ->', result);
        })
}

const map = {
    hello: helloWorldExample,
    func: functionExample,
    async: promiseExample
};


map[process.argv[2]]()