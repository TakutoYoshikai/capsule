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
const IFACE: &str = "wlp3s0";
```

**include encrypted secret key in this program in main**
```rust
//You can paste here encrypted secret key.
const ENCRYPTED_TEXT: &str = "";
```

**build**
```bash
cargo build --release
```

### License
MIT License
