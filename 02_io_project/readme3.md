
## Récupérer les arguments

- essayer de récupérer les arguments avec le code suivant:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args[1]; // ne marche pas, pourquoi ?
    let filename = args[2]; // ne marche pas, pourquoi ?

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

- quelles solutions envisager ?

> 1 solution: readme4