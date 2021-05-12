# capsule
This is a program for managing private keys safely.

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

**write your network interface in main**
```rust
let key = get_key("CHANGE HERE TO NETWORK INTERFACE.");
```

**include encrypted secret key in this program in main**
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
