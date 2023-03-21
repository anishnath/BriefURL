# Brief URL 

## Description

This is a simple URL shortener written in Rust. It uses the Rocket framework and Diesel ORM.

## SQLITE Setup 

```bash
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
cargo install --path .
diesel setup --database-url='sqlite://briefurl_db'
diesel migration generate brief_url
diesel migration run  --database-url='sqlite://briefurl_db'
```

### Contribution
https://8gwifi.org