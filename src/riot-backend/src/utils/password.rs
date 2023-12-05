use argon2::{self, Config as Argon2cfg, Variant, Version};
use log::debug;

/// RFC 9106 (OWASP) recommends using Argon2id
pub fn get_pwd_hash(salt: &[u8], password: &[u8]) -> String {
    let config = Argon2cfg {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 1,
        lanes: 1,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    argon2::hash_encoded(password, salt, &config).unwrap()
}
pub fn verify(hash: &str, password: &[u8]) -> bool {
    debug!("hash={}, pwd={:?}", hash, password);
    argon2::verify_encoded(hash, password).unwrap()
}
#[cfg(test)]
mod tests {
    use std::thread;

    use rand::Rng;

    use crate::utils::password::{get_pwd_hash, verify};

    fn generate_random_string(length: usize) -> String {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=`~";
        let mut rng = rand::thread_rng();

        let random_string: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        random_string
    }

    #[test]
    pub fn hash_benchmark() {
        let hash = get_pwd_hash(b"114514114514", b"Myp4ss!w0rD");
        println!("{}", hash.to_string());

        // Create a vector to store the thread handles
        let mut handles = vec![];

        for _ in 0..16 {
            let handle = thread::spawn(move || {
                for _ in 0..64 {
                    let pwdstr = generate_random_string(8);
                    let pwd = pwdstr.as_bytes();
                    let hash = get_pwd_hash(b"114514114514", pwd);
                    assert!(verify(&hash, pwd));
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
