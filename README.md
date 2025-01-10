# Ham OS (Rust)

## Chapter 1

I am going to write an OS in Rust. Each chapter will include my thoughts, notes, and important details.

First I need to create a baremetal binary of rust so it can run without the standard library (which uses system calls that aren't available). A typical Rust binary that links with the standard library starts execution in a C runtime library `crt0` (this creates the stack and palces args in the right registers). This baremetal executable does not have access to the Rust start point for runtime or `crt0` so I need to define an entry point for the runtime system.

`rustc --version --verbose` displays the host target system and what rust is attempting to compile against. We don't want to compile for the host system so we will use an embedded ARM chip target (`thumbv7em-none-eabihf`) to simulate compiling for a target with no OS.
This will cross-compile our executable for a bare metal target system!

```
 rustup target add thumbv7em-none-eabihf
 cargo build --target thumbv7em-none-eabihf
```

## Sources

https://os.phil-opp.com/freestanding-rust-binary/