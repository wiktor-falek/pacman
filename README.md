# Roadmap

- [x] Initializing and rendering board
- [ ] Loading maps from text files
- [ ] Player movement - detect input and suppress default terminal behaviour, 
- [ ] Colission checking
- [ ] Ghost AI
- [ ] User interface - level selection, scoreboard,...



# Releasing
<h3 style="color: orange;">
1. Build the project
</h3>

```rust
// in projects main directory
cargo build --release
```

<h3 style="color: orange;">
2. Move maps folder to target/release
</h3>


```bash
# BASH
cp maps -r target/release
```

```ps 
# CMD/PS
xcopy maps target\release\ /E
```
