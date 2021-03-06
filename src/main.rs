use std::fs;
use std::env;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;

type AesCbc = Cbc<Aes256, Pkcs7>;
const ENCRYPTED_TEXT: &str = "";

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn get_ifaces() -> Vec<String> {
    let net = Path::new("/sys/class/net");
    let entry = fs::read_dir(net).expect("Error");
    let mut ifaces = entry.filter_map(|p| p.ok())
                      .map(|p| p.path().file_name().expect("Error").to_os_string())
                      .filter_map(|s| s.into_string().ok())
                      .filter(|s| s.starts_with("e") || s.starts_with("w") || s.starts_with("b"))
                      .collect::<Vec<String>>();
    ifaces.sort();
    return ifaces;
}
fn read_file(path: &str) -> Option<Vec<u8>> {
    let mut file = File::open(path).unwrap();
    let metadata = fs::metadata(path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    match file.read(&mut buffer) {
        Ok(_) => (),
        Err(_) => return None,
    }
    return Some(buffer);
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
fn get_macaddr(if_name: &str) -> Option<String> {
    let net = Path::new("/sys/class/net");
    let iface = net.join(if_name).join("address");
    let mut file = match fs::File::open(iface) {
        Ok(f) => f,
        Err(_) => return None,
    };
    let mut macaddr = String::new();
    match file.read_to_string(&mut macaddr) {
        Ok(_) => (),
        Err(_) => return None,
    };
    return Some(macaddr);
}

fn get_key() -> String {
    let ifaces = get_ifaces();
    let mut key: String = "".to_string();
    for iface in ifaces {
        match get_macaddr(&iface) {
            Some(macaddr) => key += &macaddr,
            None => (),
        }
    }
    key.retain(|c| c != ':' && c != '\n');
    loop {
        if key.len() >= 32 {
            break;
        }
        key += "0";
    }
    return key.chars().skip(0).take(32).collect();
}

fn encrypt(key: &str, data: &[u8]) -> String {
    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_var(key.as_bytes(), iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data);
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

fn decrypt(key: &str, data: &str) -> Vec<u8> {
    let bytes = base64::decode(data).unwrap();
    let cipher = AesCbc::new_var(key.as_bytes(), &bytes[0..16]).unwrap();
    cipher.decrypt_vec(&bytes[16..]).unwrap()
}

fn save(filename: &str, data: Vec<u8>) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&data).unwrap();
    file.flush().unwrap();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let key = get_key();
    if args.len() <= 1 {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.push("data");
        let decrypted = decrypt(&key, &ENCRYPTED_TEXT);
        save(&path.into_os_string().into_string().unwrap(), decrypted);
        return;
    }
    let target: &str = &args[1];
    let data = read_file(target);
    print!("{}", encrypt(&key, &data.unwrap()));
}
