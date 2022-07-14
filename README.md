# Roadmap

- [x] Initializing and rendering board
- [ ] Loading maps from text files
- [ ] Player movement - detect input and suppress default terminal behaviour, 
- [ ] Colission checking
- [ ] Ghost AI
- [ ] User interface - level selection, scoreboard,...



# Releasing
`1. Build the project`
```rust
// in projects main directory
cargo build --release
```

`2. Move maps folder to target/release`
```bash
# BASH
cp maps -r target/release
```

```ps 
# CMD/PS
xcopy maps target\release\ /E
```