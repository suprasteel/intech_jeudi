
# Exercices

Suite de documents apportant les réponses aux questions des premiers

# Passer des arguments

- expérimenter le passage d'arguments en rust

```
cargo run argument1 example-filename.txt
```

---

- récupérer les arguments:
	- `env::args().collect()`

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

- essayer sans arguments

    - panic si utf-8 invalide
    - si besoin, utiliser std::env::args_os (OsString)
    - OsString et String
