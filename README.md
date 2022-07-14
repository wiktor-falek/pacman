# Roadmap

- [x] Initializing and rendering board
- [ ] Loading maps from text files
- [ ] Player movement - detect input and suppress default terminal behaviour, 
- [ ] Colission checking
- [ ] Ghost AI
- [ ] User interface - level selection, scoreboard,...



# Releasing
<h3>
1. Build the project
</h3>

```rust
// in projects main directory
cargo build --release
```


## 2. Move maps folder to target/release


### Bash
```bash
cp maps -r target/release
```

### Powershell/cmd

```cmd
xcopy maps target\release\ /E
```

## 3. Run executable

### Bash
```bash
./target/release/pacman
```

### Powershell/cmd

```cmd
idk
```
