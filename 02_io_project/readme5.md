## Afficher les types

```rust
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

```

## Se débarasser de clone

- Trouver une solution !

> readme6