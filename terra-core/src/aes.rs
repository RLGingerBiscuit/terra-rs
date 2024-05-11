use std::io;

use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
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

    let mut enc_data = data.to_vec();
    let padding = data.len() % 16;
    if padding > 0 {
        enc_data.extend(vec![0u8; 16 - padding]);
    } else {
        enc_data.extend(vec![0u8; 16]);
    }

    enc.encrypt_padded_mut::<Pkcs7>(&mut enc_data, data.len())
        .map_err(|err| anyhow::anyhow!(err))?;

    writer.write_all(&enc_data)?;

    Ok(())
}
