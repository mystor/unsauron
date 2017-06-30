# unsauron

> NOTE: This is not published on crates.io, as it depends on the master version
> of `syn`. It depends on this version of `syn` because the version of `syn` on
> crates.io doesn't correctly handle precedence, which is very important for the
> transformations made by this crate.
> 
> When the next version of `syn` is publsihed, I will publish `unsauron` as well.

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
