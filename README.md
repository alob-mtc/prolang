# prolang

> interpreted language

experimental prototyping langage

It supports mathematical expressions, variable bindings, functions, conditionals, return statements and even advanced concepts like higher-order functions and closures.

And then there are the different data types:

- integers
- booleans
- strings
- arrays
- hashes

### language snipet

> syntax and language grammar would change and evolve

```rs
// expressions
let one = 1;
let two = 2;
let add = fn(x, y) {
    return x + y;
};
let result = add(one, two);

// if-expression
let d = if (c > a) { return 99 } else { return 100 };

// function-expression
let addThree = fn(x) { return x + 3 };

println("x+3 = ", addThree(3));

let add = fn(a,b) { a + b};
let sub = fn(a,b) { a - b};
let applyFunc = fn(a,b,func) { func(a,b) };

applyFunc(2, 2, add);   // 4
applyFunc(10, 2, sub);  // 8

// function-closure: higher-order
let makeGreeter = fn(greeting) {
    return fn(name) {
        return greeting + " " + name + "!" \
    }
};
let hello  = makeGreeter("Hello");
hello("John");   // Hello John!

// array
let arr = [1, 2, 3, 4];
len(arr);    // 4
len("hello world");    //11

let a  = [1, 2, 3, 4];
let b = push(a, 5);

println(a) // [1, 2, 3, 4]
println(b) // [1, 2, 3, 4, 5]

a = rest(a);
println(a) // [2, 3, 4]
a = rest(a);
println(rest(a)) // [3, 4]


// self calling function
let even = fn() {
    let arr = [];
    for (i in 0..10) {
        if i % 2 == 0 {
            arr = push(arr, i);
        }
    }

    return arr;
}()

println("list of even number:", even)

// recursion
fn fib(n) {
    if n <= 1 {
        return n;
    };

    return fib(n-1) + fib(n-2);
}


// built in functions: push and rest
let map = fn(arr, f) {
    let iter = fn(arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), push(accumulated, f(arr[0])));
        }
    };
    iter(arr, []);
};

let a = [1, 2, 3, 4];
let double = fn(x) { x * 2 };
map(a, double); //[2, 4, 6, 8]

let reduce = fn(arr, initial, f) {
    let iter = fn(arr, result) {
        if (len(arr) == 0) {
            result
        } else {
            iter(rest(arr), f(result, first(arr)));
        }
    };
    iter(arr, initial);
};

let sum = fn(arr) {
    reduce(arr, 0, fn(initial, el) { initial + el });
};

sum([1, 2, 3, 4, 5]);   // 15

// loops
for {
    // action
};

let x = 0;
for (x < 10) {
    x++;
};

for (i in 0..10) {
    println(i);
};

// Hash
let myHash = {"name": "Jimmy", "age": 72, "band": "Led Zeppelin"};
myHash["name"]  // Jimmy
myHash["age"]  // 72

```

### TODO

[] - Type system
[] - GC
[] - Error handling
