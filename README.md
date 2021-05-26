# capsule
This is a program to store secret data safely. capsule file is an executable file to extract encrypted data in the capsule file. The secret data is encrypted by MAC address of your PC automatically. You should separate and store the secret data and your PC.

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
