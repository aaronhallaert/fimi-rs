# fimi-rs
🚧 **Disclaimer: work in progress** 🚧

**Fi**le **mi**grations made easy.

## Description

The purpose of this repository is to offer a framework for configuration file migrations, akin to database migrations. `Fimi` is designed to be generic, handling various file content types (e.g., JSON, YAML) without concern.
This crate does not impose any restrictions on the migration itself; it is entirely up to the programmer implementing the migration. The migration code should be able to run independently of this framework.

The `fimi` crate delivers the core business logic for this framework, while `fimi_magic` includes procedural macros to support this logic.

## Usage

The crate tracks executed migrations by keeping record in a manifest file.

```rust
/// see `fimi_example` to explore the complete example

mod migrations {
    /// the migration modules have a name convention of _*datetimeId*_*migration_name*
    mod _20241021212814_init_migration {
        use fimi::Migration;
        pub struct InitMigration;

        impl Migration for InitMigration {
            fn up(&self) -> Result<(), Box<dyn std::error::Error>> {
                // implement this
                Ok(())
            }

            fn down(&self) -> Result<(), Box<dyn std::error::Error>> {
                // implement this
                Ok(())
            }
        }
    }

    use fimi_magic::migrations;

    /// the arguments of this macro are modules
    /// each module should have a public accessible struct with the same name as the migration 
    /// that implements the `fimi::Migration` trait.
    /// (here: InitMigration & AddNewKeyToTestStructs)
    migrations!(
        _20241021212814_init_migration,
        _20241022110124_add_new_key_to_test_structs
    );
}

fn main() {
    let executor = migrations::migrations();

    executor.up();
}
```

## Roadmap

- [ ] interactive up/down with specified migrations (cargo-fimi?)
    - needs dynamically loading of the migration modules when executing from cli (e.g. with `cargo fimi`) in the project where the migrations exist 🤔
- [ ] scaffold migrations

## Similar crates

`fimi-rs` differs in the fact that the migrations are generic and are not tied to SQL / specific database connections.

- [refinery](https://github.com/rust-db/refinery)
- [dbmigrate](https://github.com/Keats/dbmigrate)
