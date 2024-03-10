#[cfg(test)]
mod test {

    use hex_literal::hex;
    // Import relevant traits
    use salsa20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
    use salsa20::Salsa20;

    #[test]
    fn test() {
        let key = [0x42; 32];
        let nonce = [0x24; 8];
        let plaintext = hex!("00010203 04050607 08090A0B 0C0D0E0F");
        let ciphertext = hex!("85843cc5 d58cce7b 5dd3dd04 fa005ded");

        // Key and IV must be references to the `GenericArray` type.
        // Here we use the `Into` trait to convert arrays into it.
        let mut cipher = Salsa20::new(&key.into(), &nonce.into());

        let mut buffer = plaintext.clone();

        // apply keystream (encrypt)
        cipher.apply_keystream(&mut buffer);
        assert_eq!(buffer, ciphertext);

        let ciphertext = buffer.clone();

        // Salsa ciphers support seeking
        cipher.seek(0u32);

        // decrypt ciphertext by applying keystream again
        cipher.apply_keystream(&mut buffer);
        assert_eq!(buffer, plaintext);

        // stream ciphers can be used with streaming messages
        cipher.seek(0u32);
        for chunk in buffer.chunks_mut(3) {
            cipher.apply_keystream(chunk);
        }
        assert_eq!(buffer, ciphertext);
    }
}
