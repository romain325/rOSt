use core::mem;

const WORD_SIZE: usize = mem::size_of::<usize>();

/// Memset
///
/// Fill a block of memory with a specified value.
///
/// This faster implementation works by setting bytes not one-by-one, but in
/// groups of 8 bytes (or 4 bytes in the case of 32-bit architectures).
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c: usize = mem::transmute([c as u8; WORD_SIZE]);
    let mut i: usize = 0;

    while i < (n/WORD_SIZE)*WORD_SIZE {
        *((dest as usize +i) as *mut usize) = c;
        i += WORD_SIZE;
    }
    let c = c as u8;
    while i < n {
        *((dest as usize +i) as *mut u8) = c;
        i += 1;
    }
    dest
}

/// Memcpy
///
/// Copy N bytes of memory from one location to another.
///
/// This faster implementation works by copying bytes not one-by-one, but in
/// groups of 8 bytes (or 4 bytes in the case of 32-bit architectures).
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i : usize = 0;
    while i < (n/WORD_SIZE)*WORD_SIZE {
        *((dest as usize +i) as *mut usize) = *((src as usize +i) as *const usize);
        i += WORD_SIZE;
    }
    while i < n {
        *((dest as usize +i) as *mut u8) = *((src as usize +i) as *const u8);
        i += 1;
    }
    dest
}

/// Memmove
///
/// Copy N bytes of memory from src to dest. The memory areas may overlap.
///
/// This faster implementation works by copying bytes not one-by-one, but in
/// groups of 8 bytes (or 4 bytes in the case of 32-bit architectures).
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        let opti_mem : usize = (n/WORD_SIZE)*WORD_SIZE;
        let mut i : usize = opti_mem;
        while i != 0 {
            i -= WORD_SIZE;
            *((dest as usize + i) as *mut usize) = *((src as usize + i) as *const usize);
        }
        i = n;
        while i != opti_mem {
            i -= 1;
            *((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
        }
    } else {
        let mut i : usize = 0;
        while i < (n/WORD_SIZE)*WORD_SIZE {
            *((dest as usize +i) as *mut usize) = *((src as usize + i) as *const usize);
            i += WORD_SIZE;
        }
        while i < n {
            *((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
            i += 1;
        }
    }
    dest
}

/// Memcmp
///
/// Compare two blocks of memory.
///
/// This faster implementation works by comparing bytes not one-by-one, but in
/// groups of 8 bytes (or 4 bytes in the case of 32-bit architectures).
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i: usize = 0;
    while i < (n/WORD_SIZE)*WORD_SIZE {
        let a = *((s1 as usize + i) as *const usize);
        let b = *((s2 as usize + i) as *const usize);
        // check if the blocks are the same
        if a != b {
            let n : usize = i + WORD_SIZE;
            while i < n {
                let a = *((s1 as usize + i) as *const u8);
                let b = *((s2 as usize + i) as *const u8);
                if a != b {
                    // return difference between the mem blocks
                    return a as i32 - b as i32;
                }
                i += 1;
            }
        }
        i += WORD_SIZE;
    }

    while i < n {
        let a = *((s1 as usize + i) as *const u8);
        let b = *((s2 as usize + i) as *const u8);
        if a != b {
            // return difference between the mem blocks
            return a as i32 - b as i32;
        }
        i += 1;
    }
    // return 0 if no differences are found
    0
}

#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    memcmp(s1,s2,n)
}
