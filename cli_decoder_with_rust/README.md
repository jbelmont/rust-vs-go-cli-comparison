# JWT Decoder

This is a simple rust program that decodes a json web token.

You can read more about [JWTs in RFC 7519](https://tools.ietf.org/html/rfc7519)

## Building the Rust Program

To build the project you need to run `cargo build`

## Running the Rust Program

You can run the program by issuing a command like this:

```bash
cargo run $(go run generate_token.go)
```

The part `$()` is command substitution in bash and essentially executes a little go program that I wrote that creates a JWT that is deserialized into the type specified in *src/main.rs*

If you don't have GO installed then you can simply run this in the command line:

```bash
cargo run eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJCYWRhc3NlcyIsImV4cCI6MTMwMDgxOTM4MCwiaWF0IjoxMzAwODE5MzgyLCJpc3MiOiJKb2huIFJhbWJvIiwianRpIjoib25lUmFuZG9tZVN0cmluZzEyMzQ1NiIsIm5iZiI6MTMwMDgxOTM4MSwic3ViIjoiTHVrZSBDYWdlIn0.7DtGRRz6YGc8hKr8o8ll6cIx7MRhyQTWpAgxz25cxyw
```

which is the JWT that the go program generates
