# capsule
This is a program for manage private keys safely.

### Usage
**git clone**
```bash
git clone https://github.com/TakutoYoshikai/capsule.git
```

**encrypt secret key**
```bash
# You can copy this output.
cargo run /path/to/secret-key
```

**include encrypted secret key in this program**
```rust
//You can paste encrypted secret key.
let text = "CHANGE HERE";
```

**build**
```bash
cargo build --release
```

### License
MIT License
