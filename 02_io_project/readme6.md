
## Se débarasser de clone

```rust
use std::env;
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    dbg!(&query);
    dbg!(&filename);

    print_type_of(&query);
    print_type_of(&filename);

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

---

## Garder le code dont on a besoin

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

---

```
$ cargo run test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt
```

---

## Lire un fichier

`poem.txt`

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

by Emily Dickinson

---

- use std::fs
- fs::read_to_string -> Result<String>

```rust
use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

- que se passe-t-il si le code est lancé sans arguments ?
- que se passe-t-il si le fichier n'est pas trouvé ?

- compile et affiche le poeme

---

Reprendre eventuellement https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html