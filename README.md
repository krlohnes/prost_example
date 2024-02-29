# Prost Example

This is a sample rust application showing how to build and use protobufs with prost in rust

## Build requirements
[protoc installation instructions](https://grpc.io/docs/protoc-installation/)

Just in case the link goes dead, here's the instructions

Linux, using apt or apt-get, for example:
```
$ apt install -y protobuf-compiler
$ protoc --version  # Ensure compiler version is 3+
```

MacOS, using Homebrew:
```
$ brew install protobuf
$ protoc --version  # Ensure compiler version is 3+
```

## build.rs
The build.rs is the main entry point for building the protobuf files. For this example, we're using
the `walkdir` crate to get the full path for all the `.proto` files. Normally, you'd specify all of
the files in the first slice, but I'm lazy like that.

## Using the protobufs
You'll need to recreate your package structure using `mod` blocks. It's useful to do this in a
`lib.rs` for your project. See `src/lib.rs` for an example.

##Generated code location
`debug/build/prost_example-{}/out/`

The build directory varies.
