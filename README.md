## Workspace Layout:

### Encoder:
* Functionality for encoding/decoding strings, vectors, and functions.
* Showcases modules, as well as more advanced trait usage and the use of closures for functional development.
* Custom Iterator implementation for finer grain control of keystream generation.

### Payload Generator:
* Shows unsafe code, and demostrates that you can do anything in Rust that you could in C, it just takes more effort/boilerplate.

### Payload Executor:
* Uses the encoder/decoder to execute a payload created from the generator. 
