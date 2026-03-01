use std::io;

use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, BlockSizeUser, KeyIvInit},
    Aes128Dec, Aes128Enc,
};
use cbc::{Decryptor, Encryptor};

use crate::ENCRYPTION_BYTES;

pub(crate) fn decrypt_from_reader<R: io::Read>(mut reader: R) -> anyhow::Result<Vec<u8>> {
    let dec = Decryptor::<Aes128Dec>::new(ENCRYPTION_BYTES.into(), ENCRYPTION_BYTES.into());

    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    dec.decrypt_padded_mut::<Pkcs7>(&mut data)
        .map_err(|err| anyhow::anyhow!(err))?;

    Ok(data)
}

pub(crate) fn encrypt_to_writer<W: io::Write>(mut writer: W, data: &[u8]) -> anyhow::Result<()> {
    let enc = Encryptor::<Aes128Enc>::new(ENCRYPTION_BYTES.into(), ENCRYPTION_BYTES.into());

    let length = if data.len().is_multiple_of(Aes128Enc::block_size()) {
        data.len() + Aes128Enc::block_size()
    } else {
        data.len() + (Aes128Enc::block_size() - (data.len() % Aes128Enc::block_size()))
    };

    let mut enc_data = vec![0u8; length];
    enc_data[..data.len()].copy_from_slice(data);

    enc.encrypt_padded_mut::<Pkcs7>(&mut enc_data, data.len())
        .map_err(|err| anyhow::anyhow!(err))?;

    writer.write_all(&enc_data)?;

    Ok(())
}
