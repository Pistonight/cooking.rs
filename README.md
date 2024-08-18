cooking.rs
----------

Cooking simulator for BotW


Example
-------

Example from `src/cli.rs` which is the default binary along side the library

```rust
use cooking::Cook;

fn main() {
    let cook = Cook::new();
    let recipe = cook.cook(&["Apple"]);
    println!("{:?}", recipe);

    let recipe = cook.cook(&["Fairy"]);
    println!("{:?}", recipe);

    let items = ["Apple", "Fairy", "Swift Carrot", "Apple"];
    let recipe = cook.cook(&items);
    println!("{:?}", recipe);
}
```

License
-------

BSD 2-Clause
