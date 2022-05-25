# Rust-Restaurant-Api-Backend

Server for the [Rust restaurant client](https://github.com/Dszedlak/rust_restaurant_client/blob/master/README.md "Restaurant Client"), written in Rust using Rocket and Sqlx.

#### Libraries used:

[Rocket](https://crates.io/crates/rocket "Rocket.rs")

[Sqlx](https://crates.io/crates/sqlx "Sqlx")

[Serde](https://crates.io/crates/serde "Serde")

[Log](https://crates.io/crates/simplelog "Simplelog")

[thiserror](https://crates.io/crates/thiserror "thiserror")

[rand](https://crates.io/crates/rand "rand")

[chrono](https://crates.io/crates/chrono "chrono")

## Prerequisites

***Ensure latest version of rust is installed***

## Getting started

1. Download/checkout the repository:
   
    ```gh repo clone Dszedlak/Rust-Restaurant-Api-Backend```
  
    ```https://github.com/Dszedlak/Rust-Restaurant-Api-Backend.git```
  
 2. Once downloaded/extracted, open a terminal in the 'Rust-Restaurant-Api-Backend' folder.
 3. Once you have a terminal inside the *Rust-Restaurant-Api-Backend* folder, type **Cargo run**, and hit **Enter**
 4. Allow some time for it to install necessary dependencies and build the server.
 5. Once completed, you should see *Running `target\debug\rust_restaurant_api.exe`* in your terminal. Additionally, to check for successful execution of the rust_restaurant_api.exe binary, check the 'server log'.

Additionally, as a quality of life change, add a enviornment variable **ROCKET_CLI_COLORS="0"**, for a better log viewing experience.
## Workflows

Unit testing, Code linting, Build execution and security workflows.

### Improvements
- Check if an item has already been cooked before deleting an order with a preparation time, if so dont allow it.
- Server does not enforce non-allowing of duplicate items in an order. It should check if the item is used twice and could correct it by appropriately increasing the amount.

### Unit Tests

To manually test unit tests, type ```cargo test```. All 6 tests should pass.
