# prolang

> interpreted language

experimental prototyping language

It supports mathematical expressions, variable bindings, functions, conditionals, return statements and even advanced concepts like higher-order functions and closures.

And then there are the different data types:

- integers => int, float
- booleans => bool: true, false
- strings => str: ""
- char => ''
- arrays => [T], T: (int|bool|...)
- hashes => {"": ""}
- struct => struct {}
- enum => enum(x,y,z), enum(x(int), y(float), z(bool))
- interface => interface { run(): number; fn }

### language snipet

> syntax and language grammar would change and evolve

Run examples

```sh
$ cargo run example/main.pr
```

Run repl (interactive mode)

```sh
$ cargo run
```

##### If-expression

```rs
let d: int = if (c > a) { 99 } else { 100 };
```

##### Function-expression & function-closure: higher-order

```rs
let add = fn(a,b): int { a + b};
let sub = fn(a,b): int { a - b};
let applyFunc = fn(a: int, b: int ,func: fn(int, int): int): int { func(a,b) };

applyFunc(2, 2, add); // 4
applyFunc(10, 2, sub); // 8

let makeGreeter: fn(str): str = fn(greeting: str): fn(str): str {
    return fn(name: str): str {
        return greeting + " " + name + "!"
    }
};

let hello = makeGreeter("Hello");
hello("John"); // Hello John!
```

##### Array

```rs
let arr: []int = [1, 2, 3, 4];
len(arr); // 4
len("hello world"); //11

let a: []int = [1, 2, 3, 4];
let b: []int = push(a, 5);

println(a) // [1, 2, 3, 4]
println(b) // [1, 2, 3, 4, 5]

a = rest(a);
println(a) // [2, 3, 4]
a = rest(a);
println(rest(a)) // [3, 4]
```

##### Self calling function

```rs
let even: []int = fn(): []int {
    let arr: []int = [];
    for (i in 0..10) {
        if i % 2 == 0 {
            arr = push(arr, i);
        }
    }
    return arr;
}()

println("list of even number:", even)

```

##### Recursion

```rs
let fib: float = fn(n): float {
    if n <= 1 {
        return n;
    };
    return fib(n-1) + fib(n-2);
}

```

##### Built in functions: push and rest

```rs
// create map function with built in function
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

// create reduce function with built in function
let reduce = fn(arr, initial, f) {
    let iter = fn(arr, result) {
        if (len(arr) == 0) {
            result
        } else {
            iter(rest(arr), f(result, arr[0]));
        }
    };
iter(arr, initial);
};

let sum = fn(arr) {
    reduce(arr, 0, fn(initial, el) { initial + el });
};
sum([1, 2, 3, 4, 5]); // 15

```

##### Loops

```rs
for {
    // action
};

let x: int = 0;
for (x < 10) {
    x = x + 1; // TODO: implement "+=", "-="
};

for (i in 0..10) {
    println(i);
};

```

##### Hash

```rs
let myHash: = {"name": "Jimmy", "age": 72, "band": "Led Zeppelin"};
myHash["name"] // Jimmy
myHash["age"] // 72

```

##### enum

```rs

let Event: enum = enum(add, remove, create, key_press(str))
println(Event.add);
println(Event.remove);
println(Event.created);

let Option = enum(some(num), none)

Option.some(1)


let age: num = 2;
let name: str = "bola";
```

### TODO:

##### Frontend

- [ ] Type system

##### Backend

- [ ] GC
- [ ] Error handling
- [ ] Async and concurrency
- [ ] std lib
