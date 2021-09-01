/// # Libc `memset` implemetation
///
/// ## Prototype
/// void *memset(void *s, int c, size_t n);
///
/// ## Description
/// The 'memset()' function fills the first n bytes of the memory area 
///   pointed to by s with the constant byte c.
///
/// ## Returns
/// The 'memset()' function returns a pointer to the memory area s.
#[no_mangle]
pub unsafe fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    asm!(
        "str",
        inout("rdi") s => _,
        in("eax") c as u32,
        inout("rcx") n => _
    );
    s
}

/// # Libc `memcpy` implementation
///
/// ## Prototype
/// void *memcpy(void *restrict dest, const void *restrict src, size_t n);
///
/// ## Description
/// The `memcpy()` function copies n bytes from memory area src to
///   memory area dest.  The memory areas must not overlap.  Use
///   `memmove` if the memory areas do overlap.
///
/// ## Return value
/// The `memcpy()` function returns a pointer to dest.
#[no_mangle]
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    asm!(
        "rep movsb",
        inout("rsi") src => _,
        inout("rdi") dest => _,
        in("rcx") n
    );
    dest
}

/// # Libc `memcmp` implementation
///
/// ## Prototype
/// int memcmp(const void *s1, const void *s2, size_t n);
///
/// ## Description
/// The 'memcmp()' function compares the first n bytes (each interpreted
/// as unsigned char) of the memory areas s1 and s2.
///
/// ## Return value
/// The 'memcmp()' function returns an integer less than, equal to, or 
///   greater than zero if the first n bytes of s1 is found, 
///   respectively, to be less than, to match, or be greater 
///   than the first n bytes of s2.
/// For a nonzero return value, the sign is determined by the sign 
///   of the difference between the first pair of bytes (interpreted 
///   as unsigned char) that differ in s1 and s2.
///
/// TODO: Better (faster) memcmp implementation
#[no_mangle]
pub unsafe fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    /*asm!(
        "repe cmpsb",
        in("rsi") s1,
        in("rdi") s2,
        in("rcx") n,
    );
    0*/

    let mut i = 0;
    while i < n {
        let s1o = *s1.offset(i as isize);
        let s2o = *s2.offset(i as isize);
        if s1o != s2o {
            return s1o as i32 - s2o as i32
        }
        i += 1;
    }
    0
}
