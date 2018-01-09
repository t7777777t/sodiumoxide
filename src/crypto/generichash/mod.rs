
#[cfg(not(feature = "std"))] use prelude::*;
use ffi;
//use libc::c_ulonglong;

/// Mininum number of bytes in a `GenericHash`.
pub const BYTES_MIN: usize = ffi::crypto_generichash_BYTES_MIN;

/// Maximum number of bytes in a `GenericHash`.
pub const BYTES_MAX: usize = ffi::crypto_generichash_BYTES_MAX;
pub const BYTES: usize = ffi::crypto_generichash_BYTES;
pub const KEYBYTES_MIN: usize = ffi::crypto_generichash_KEYBYTES_MIN;
pub const KEYBYTES_MAX: usize = ffi::crypto_generichash_KEYBYTES_MAX;
pub const KEYBYTES: usize = ffi::crypto_generichash_KEYBYTES;
pub const PRIMITIVE: &'static str = "blake2b";

new_type! { 
    /// `GenericHash` is the result of a generic hash function
    public GenericHash(BYTES);
}

pub fn generichash(in_: &[u8], key: &[u8]) -> Result<GenericHash, ()> {
    let mut out = [0u8; BYTES];
    if unsafe { ffi::crypto_generichash(out.as_mut_ptr(), out.len(),
                                   in_.as_ptr(), in_.len() as u64,
                                   key.as_ptr(), key.len())
    } == 0 { 
        Ok(GenericHash(out))
    } else {
        Err(())
    }
}

use ffi::crypto_generichash_state;
pub fn generichash_init(state: &mut crypto_generichash_state, key: &[u8]) -> Result<usize, ()> {
    Ok(0)
}
pub fn generichash_update(state: &mut crypto_generichash_state, in_: &[u8]) -> Result<(), ()> {
    Ok(())
}   
pub fn generichash_final(state: &mut crypto_generichash_state) -> Result<Vec<u8>, ()> {
    Ok(vec![])
}
  
mod test { 
    use super::*;

    #[test]
    fn test_generichash() { 
        assert!(64 > BYTES_MIN);
        println!("{}", BYTES_MAX);
        assert!(64 == BYTES_MAX);
        let m = [0u8; 64];
        let key = [0u8; KEYBYTES];
        let hash_expected = GenericHash([0x00; 32]);
        let r = generichash(&m, &key).unwrap(); 
        assert_eq!(r, hash_expected);
    }

    #[test]
    fn test_generichash_to_vec() {
        let hash = GenericHash([0x00; 32]);
        let mut x: Vec<u8> = Vec::new();
        x.extend_from_slice(&hash[..]);
        assert_eq!(BYTES, x.len());
    }
}
    
