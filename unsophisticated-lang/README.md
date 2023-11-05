# S-Expression Interpreter in Rust

This is a simple S-Expression interpreter written in Rust.

## Usage (Not fixed yet)

```common-lisp
(
    (define fact
        (lambda (n)
        (if (<= n 1)
            1
            (* n (fact (- n 1))))))
    
    (fact 10))
```