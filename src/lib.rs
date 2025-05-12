use libc::{c_int, c_ulong};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn get() -> usize {
    10
}

#[link(name = "z")]
unsafe extern "C" {
    fn compress(dest: *mut u8, dest_len: *mut c_ulong,
                source: *const u8, source_len: c_ulong) -> c_int;
    fn compressBound(source_len: c_ulong) -> c_ulong;
    fn uncompress(dest: *mut u8, dest_len: *mut c_ulong, source: *const u8, source_len: c_ulong) -> c_int;
}

pub fn zlib_compress(source: &[u8]) -> Vec<u8>  {
    unsafe {
        let source_len = source.len() as c_ulong;
        let mut dest_len = compressBound(source_len);
        let mut dest = Vec::with_capacity(dest_len as usize);
        compress(dest.as_mut_ptr(), &mut dest_len, source.as_ptr(), source_len);
        dest.set_len(dest_len as usize);
        dest
    }
}

pub fn zlib_uncompress(source: &[u8]) -> Vec<u8>  {
    unsafe {
        let source_len = source.len() as c_ulong;
        let mut dest_len = source_len;
        let mut dest = Vec::with_capacity(dest_len as usize);
        uncompress(dest.as_mut_ptr(), &mut dest_len, source.as_ptr(), source_len);
        dest.set_len(dest_len as usize);
        dest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
