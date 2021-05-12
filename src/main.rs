use std::fs;
use std::env;
use std::io::Read;
use std::path::Path;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;

type AesCbc = Cbc<Aes256, Pkcs7>;

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn read_file(path: &str) -> String {
    return fs::read_to_string(path).unwrap();
}
fn gen_ascii_chars(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect()
    ).unwrap()
}
fn get_macaddr(if_name: &str) -> String {
    let net = Path::new("/sys/class/net");
    let iface = net.join(if_name).join("address");
    let mut f = fs::File::open(iface).unwrap();
    let mut macaddr = String::new();
    f.read_to_string(&mut macaddr).unwrap();
    return macaddr;
}

fn get_key(if_name: &str) -> String {
    return get_macaddr(if_name) + "00000000000000";
}

fn encrypt(key: &str, data: &str) -> String {
    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_var(key.as_bytes(), iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

fn decrypt(key: &str, data: &str) -> String {
    let bytes = base64::decode(data).unwrap();
    let cipher = AesCbc::new_var(key.as_bytes(), &bytes[0..16]).unwrap();
    String::from_utf8(cipher.decrypt_vec(&bytes[16..]).unwrap()).unwrap()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let text = "CHANGE HERE";
    let key = get_key("CHANGE HERE TO NETWORK INTERFACE.");
    if args.len() <= 1 {
        print!("{}", decrypt(&key, &text));
        return;
    }
    let target: &str = &args[1];
    let text = read_file(target);
    print!("{}", encrypt(&key, &text));
}
