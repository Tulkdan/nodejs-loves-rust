const myFunc = (n1, n2) => n1 + n2;

const fibonacci = async (n) => {
    let last = 1;
    let secondLast = 0;

    for (let x = 2; x < n; x++) {
        const temp = last;
        last = last + secondLast;
        secondLast = temp
    }

    return last;
}

function helloWorldExample() {
    console.log('[HelloWorldExample] hello', 'world!');
}

function functionExample() {
    const sumNumbers = myFunc(10, 10);
    console.log('[FunctionExample] myFunc(10, 10)', sumNumbers);
}

function promiseExample() {
    fibonacci(10)
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
