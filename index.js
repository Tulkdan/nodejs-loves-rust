const { promisify } = require('util');
const addon = require('./index.node');

const fibonacci = async value => {
    let f0 = 0;
    let f1 = 1;
    for (let x = 1; x < value; x++) {
        let f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
    }
    return f0;
}

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

    const fibAsync = promisify(myAsyncFunc)

    console.time('fibonacci')
    fibAsync(1000)
        .then((result) => console.log('[PromiseExample] myAsyncFunc(1000)', result))
        .catch((error) => console.error('error', error))
        .finally(() => console.timeEnd('fibonacci'))
}

function pureJs() {
    console.time('fibonacci')
    fibonacci(1000)
        .then((result) => console.log('[PromiseExample] myAsyncFunc(1000)', result))
        .catch((error) => console.error('error', error))
        .finally(() => console.timeEnd('fibonacci'))
}


const map = {
    hello: helloWorldExample,
    func: functionExample,
    async: promiseExample,
    js: pureJs,
};

map[process.argv[2]]()