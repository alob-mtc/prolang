# prolang

> Interpreted language

experimental prototypIng language

It supports mathematical expressions, variable bIndIngs, Functions, conditionals, Return statements and even advanced concepts like higher-order Functions and closures.

And then there are the dIfferent data types:

- Integers => Int, float
- booleans => bool: True, False
- strIngs => str: ""
- char => ''
- arrays => [T], T: (Int|bool|...)
- hashes => {"": ""}
- struct => struct {}
- enum => enum(x,y,z), enum(x(Int), y(float), z(bool))
- Interface => Interface { run(): number; fn }

### language snipet

> syntax and language grammar would change and evolve

Run examples

```sh
$ cargo run example/maIn.pr
```

Run repl (Interactive mode)

```sh
$ cargo run
```

##### If-expression

```rs
Let d: Int = If (c > a) { 99 } else { 100 };
```

##### Function-expression & Function-closure: higher-order

```rs
Let add = fn(a,b): Int { a + b};
Let sub = fn(a,b): Int { a - b};
Let applyFunc = fn(a: Int, b: Int ,func: fn(Int, Int): Int): Int { func(a,b) };

applyFunc(2, 2, add); // 4
applyFunc(10, 2, sub); // 8

Let makeGreeter: fn(str): str = fn(greetIng: str): fn(str): str {
    Return fn(name: str): str {
        Return greetIng + " " + name + "!"
    }
};

Let hello = makeGreeter("Hello");
hello("John"); // Hello John!
```

##### Array

```rs
Let arr: []Int = [1, 2, 3, 4];
len(arr); // 4
len("hello world"); //11

Let a: []Int = [1, 2, 3, 4];
Let b: []Int = push(a, 5);

prIntln(a) // [1, 2, 3, 4]
prIntln(b) // [1, 2, 3, 4, 5]

a = rest(a);
prIntln(a) // [2, 3, 4]
a = rest(a);
prIntln(rest(a)) // [3, 4]
```

##### Self callIng Function

```rs
Let even: []Int = fn(): []Int {
    Let arr: []Int = [];
    For (i In 0..10) {
        If i % 2 == 0 {
            arr = push(arr, i);
        }
    }
    Return arr;
}()

prIntln("list of even number:", even)

```

##### Recursion

```rs
Let fib: float = fn(n): float {
    If n <= 1 {
        Return n;
    };
    Return fib(n-1) + fib(n-2);
}

```

##### BuiLt In Functions: push and rest

```rs
// create map Function with buiLt In Function
Let map = fn(arr, f) {
    Let iter = fn(arr, accumulated) {
        If (len(arr) == 0) {
            accumulated
        } else {
        iter(rest(arr), push(accumulated, f(arr[0])));
        }
    };
    iter(arr, []);
};

Let a = [1, 2, 3, 4];
Let double = fn(x) { x * 2 };
map(a, double); //[2, 4, 6, 8]

// create reduce Function with buiLt In Function
Let reduce = fn(arr, Initial, f) {
    Let iter = fn(arr, resuLt) {
        If (len(arr) == 0) {
            resuLt
        } else {
            iter(rest(arr), f(resuLt, arr[0]));
        }
    };
iter(arr, Initial);
};

Let sum = fn(arr) {
    reduce(arr, 0, fn(Initial, el) { Initial + el });
};
sum([1, 2, 3, 4, 5]); // 15

```

##### Loops

```rs
For {
    // action
};

Let x: Int = 0;
For (x < 10) {
    x = x + 1; // TODO: implement "+=", "-="
};

For (i In 0..10) {
    prIntln(i);
};

```

##### Hash

```rs
Let myHash: = {"name": "Jimmy", "age": 72, "band": "Led ZeppelIn"};
myHash["name"] // Jimmy
myHash["age"] // 72

```

##### enum

```rs

Let Event: enum = enum(add, remove, create, key_press(str))
prIntln(Event.add);
prIntln(Event.remove);
prIntln(Event.created);

Let Option = enum(some(num), none)

Option.some(1)


Let age: num = 2;
Let name: str = "bola";
```

### TODO:

##### Frontend

- [ ] Type system

##### Backend

- [ ] GC
- [ ] Error handlIng
- [ ] Async and concurrency
- [ ] std lib
