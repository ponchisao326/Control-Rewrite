use fernet::{DecryptionError, Fernet};
use ring::{pbkdf2};
use data_encoding::BASE64URL_NOPAD;
use std::num::NonZeroU32;

pub struct LogEncryption {
    fernet: Fernet,
}

impl LogEncryption {
    /// Crea una nueva instancia del encriptador dada una password

    pub fn new(password: &str) -> Self {
        // Parametros PBKDF2
        let salt = b"fixed_salt_for_logs";
        let iterations = NonZeroU32::new(100_000).unwrap();
        let mut key = [0u8; 32]; // Tamaño de la clave para Fernet

        // Derivar 32 bytes con SHA256
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            salt,
            password.as_bytes(),
            &mut key,
        );

        // Codificamos en base64 URL-safe sin padding
        let key_base64 = BASE64URL_NOPAD.encode(&key);

        // Inicializamos Fernet con la clave derivada
        let fernet = Fernet::new(&key_base64).expect("Clave Invalidad");

        LogEncryption { fernet }
    }

    /// Cifra un mensaje y devuelve el token en base64-URL
    pub fn encrypt(&self, message: &str) -> String {
        self.fernet.encrypt(message.as_bytes())
    }

    /// Descifra un token y devuelve el mensaje original
    pub fn decrypt(&self, token: &str) -> Result<String, DecryptionError> {
        let decrypted = self.fernet.decrypt(token)?;
        Ok(String::from_utf8(decrypted).expect("Los bytes decifrados no son UTF-8"))
    }
}