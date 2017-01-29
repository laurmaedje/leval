# leval

Evaluate an infix notated expression.
The function uses the Shunting-Yard-Algorithm to convert the expression into RPN-Form. Then it is calculated.
Integers and floating point numbers are supported, the available operators are `+, -, *, /, ^`.
Besides the functions `sqrt(x), ln(x), sin(x), cos(x), tan(x)`, aswell as the constants `pi, e` can be used.

## Examples
```rust
assert_eq!(evaluate("12"), Ok(12.0));
assert_eq!(evaluate("2+5*3"), Ok(17.0));
assert_eq!(evaluate("2^(3+5)"), Ok(256.0));
assert_eq!(evaluate("10/5"), Ok(2.0));
assert_eq!(evaluate("4^2 * 1.8"), Ok(28.8));
```
```rust
assert_eq!(evaluate("ln(1)"), Ok(0.0));
assert_eq!(evaluate("sqrt(4*10-4)"), Ok(6.0));
assert_eq!(evaluate("(cos(ln(1)+sqrt(1) - 1))*2"), Ok(2.0));
```
