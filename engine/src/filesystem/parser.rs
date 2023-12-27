use std::ffi::CString;

//this might not belong in this mod
///Creates a whitespace CString of a given length.
pub fn create_whitespace_cstring(len:usize) -> CString {
  let mut buffer:Vec<u8> = Vec::with_capacity(len as usize + 1);
  buffer.extend([b' '].iter().cycle().take(len as usize));
  unsafe { CString::from_vec_unchecked(buffer) }
}
