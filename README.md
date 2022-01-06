# bitgen
A rust crate to (hopefully in the future) have types that are bit aligned to eachother. This means that you can have a [bool; 8] in one byte.

The most important struct in this crate is `Bit`. With it you can compress stuff to the bit level. 

For example 

```rust
fn main() {
  let tuple = (true, false, true);
  let bit_tuple = Bit::from(tuple);
  assert_eq!(mem::size_of_val(tuple), 3);
  assert_eq!(mem::size_of_val(bit_tuple), 1);
}
```

This crate also has an optional derive feature, to get a derive macro for BitType. You can derive this on structs and enums that only contain other BitType.

On the bit level the maximum amount of wasted bits is 7 bits. 
