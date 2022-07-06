## Récupérer les arguments en clonant - solution 1

```rust
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args[1].clone();
    let filename = args[2].clone();

    println!("Searching for {}", query); // pourquoi pas de clone ici ?
    println!("In file {}", filename); // pourquoi pas de clone ici ?
}
```

## Afficher plus d'informations

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args[1].clone();
    let filename = args[2].clone();

    dbg!(query.clone());
    dbg!(filename.clone());

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

## Afficher les types

- creer une fonction pour afficher le type
- on dispose de la fonction `type_name::<T>()`
- mais elle est statique
- écrire une fonction qui s'adapte au type passé pour appeler  `type_name::<T>()`

```rust
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args[1].clone();
    let filename = args[2].clone();

    dbg!(query.clone());
    dbg!(filename.clone());

    // afficher le type de query
    // afficher le type de filename

    // indice 1: utiliser std::any::type_name;
    // indice 2: comment faire inférer au compilateur le type sans le mettre en dur
    // indice 3: fonction générique (prends un argument qu'elle n'utilise pas)

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

- quelles solutions envisager ?

> solution: readme5