use aes::{Aes256Enc, Aes256Dec, cipher::{
  BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
  generic_array::GenericArray,
}, Aes256};
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

pub struct AES256CBC {
  key_arr: [u8; 32],
  iv_arr: [u8; 16],
  cipher: Aes256,
  base64_engine: engine::GeneralPurpose,
}

impl AES256CBC {
  pub fn init(key_arr: &str, iv_arr: &str) -> AES256CBC {
    // FIXME
    if key_arr.len() != 64 {
      panic!("key string must be 64 bytes")
    }
    if iv_arr.len() != 32 {
      panic!("iv string must be 32 bytes")
    }
    let key_arr = Self::str_to_arr(key_arr).try_into().unwrap();
    let iv_arr = Self::str_to_arr(iv_arr).try_into().unwrap();
    let cipher = Aes256::new(&GenericArray::from(key_arr));
    AES256CBC {
      key_arr,
      iv_arr,
      cipher,
      base64_engine: engine::GeneralPurpose::new(
        &alphabet::STANDARD,
        general_purpose::PAD,
      ),
    }
  }

  fn str_to_arr(str: &str) -> Vec<u8> {
    (0..str.len())
    .step_by(2)
    .map(|i|
      u8::from_str_radix(&str[i..i + 2], 16)
      .unwrap()
    )
    .collect::<Vec<u8>>()
  }

  pub fn encrypt(&self, arr: Vec<u8>) -> String {
    let mut target = if arr.len() != 0 { arr
      .chunks(16)
      .map(|block| {
        let mut tmp: [u8; 16] = [0u8; 16];
        let padding = (16 - arr.len()) as u8;
        for i in 0..16 {
          if i < block.len() {
            tmp[i] = block[i];
          } else {
            tmp[i] = padding;
          }
        }
        tmp
      })
      .collect::<Vec<_>>()
    } else {
      vec![[16u8; 16]]
    };
    let mut rtn: Vec<u8> = vec![];
    for i in 0..target.len() {
      let mut tmp = [0u8; 16];
      let origin = target[i];
      let to = if i == 0 { self.iv_arr } else { target[i - 1] };
      for (i, (v1, v2)) in origin.iter().zip(to.iter()).enumerate() {
          tmp[i] = v1 ^ v2;
      }
      let mut block = GenericArray::from(tmp);
      self.cipher.encrypt_block(&mut block);
      for (i, v) in block.iter().enumerate() {
        tmp[i] = *v;
        rtn.push(*v);
      }
      target[i] = tmp;
    }
    self.base64_engine.encode(rtn)
  }

  fn decrypt(&self, arr: Vec<u8>) -> Vec<u8> {
    arr
  }
}

#[cfg(test)]
mod AES256CBC_test {

  #[test]
  fn test_encrypt() {
    let key_arr = "564241314E55533654385549443249364F424637313538383939435434463343";
    let iv_arr = "564241314e5553365438554944324936";

    let aes256cbc = AES256CBC::init(key_arr, iv_arr);

    let test = aes256cbc.encrypt(vec!['a' as u8]);
    // req_body.insert("REQSITE", "x02PG4EcdFrHKluSEQQh4A==");
    // req_body.insert("Language", "zqWM417GS6dxQ7CIf65+iA==");
    // req_body.insert("TheaterCd", "LMP+XuzWskJLFG41YQ7HGA==");
    // req_body.insert("PlayYMD", "7/GCKiASBqGsIJWKmBujUA==");
    // req_body.insert("ScreenCd", "puE6q/PuILVnVlbgI8uHnA==");
    // req_body.insert("PlayNum", "hlrIVsrgDYMr7PQdmwAA4w==");
    assert_eq!("x02PG4EcdFrHKluSEQQh4A==", aes256cbc.encrypt("CJSYSTEMS".as_bytes().to_vec()));
    assert_eq!("zqWM417GS6dxQ7CIf65+iA==", aes256cbc.encrypt("kor".as_bytes().to_vec()));
    assert_eq!("LMP+XuzWskJLFG41YQ7HGA==", aes256cbc.encrypt("0013".as_bytes().to_vec()));
    assert_eq!("7/GCKiASBqGsIJWKmBujUA==", aes256cbc.encrypt("20230107".as_bytes().to_vec()));
    assert_eq!("puE6q/PuILVnVlbgI8uHnA==", aes256cbc.encrypt("018".as_bytes().to_vec()));
    assert_eq!("hlrIVsrgDYMr7PQdmwAA4w==", aes256cbc.encrypt("3".as_bytes().to_vec()));
  }
}