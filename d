#!/bin/fish
# https://stackoverflow.com/questions/61417452/how-to-get-a-feature-requirement-tag-in-the-documentation-generated-by-cargo-do#comment117282743_61417700
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features
echo '*** http://localhost:5000/uuid32/ ***'
caddy file-server --root ./target/doc --listen :5000
