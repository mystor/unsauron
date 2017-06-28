# unsauron

Sometimes it's really annoying when you have to write

```rust
&(&a + &b) + &(&c * &d)
```

when writing complex expressions with expensive-to-copy operands. Until rust
gets the ability to autoref in operators, this is our next best solution:

```rust
unsauron!({
    let result = a + b + c * d;
    result * 15
})
// Which translates to
{
    let result = &(&(a) + &(b)) + &(&(c) * &(d));
    &(result) * &(15)
}
```

## Usage


```
#[macro_use]
extern crate unsauron;

fn main() {
    unsauron!(/* Your expressions here */)
}
```
