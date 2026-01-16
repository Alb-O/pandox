---
title: "Code Blocks"
category: "Demo"
order: 5
---

# Code Blocks

Syntax-highlighted code blocks with language support.

## Rust

```rust
fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("{}", num);
    }
}
```

## JavaScript

```javascript
function greet(name) {
    console.log(`Hello, ${name}!`);
}

greet("World");
```

## Python

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
```

## Shell

```sh
trunk serve --public-url /
```
