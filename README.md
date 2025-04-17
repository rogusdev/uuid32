# uuid32

Uuids formatted as Strings in Crockford base32 in Rust. Primarily for easy to copy ids in URLs, while retaining Uuid benefits.

In short, it just replaces the canonical hexadecimal formatting for displaying Uuids.

Just use `Uuid32` anywhere you might otherwise use `Uuid` and it will pass the enclosed Uuid as a shorter String in json, display it more compactly in logs, etc.

In libraries and applications that use Uuid32s for talking to a postgres database, add feature `postgres`.

Linking the [crate](https://crates.io/crates/uuid32) and the [documentation](https://docs.rs/uuid32/latest/uuid32/) for reference.
