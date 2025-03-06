use std::ffi::c_void;

#[unsafe(no_mangle)]
pub extern "C" fn spawn_rand_gen(in_seed: u64) -> *mut c_void {
    let rng = Box::new(RandGen::new(in_seed));
    Box::into_raw(rng) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn free_rand_gen(rng: *mut c_void) {
    if !rng.is_null() {
        unsafe {
            let _ = Box::from_raw(rng as *mut _);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_u64(rng: *mut c_void) -> u64 {
    if rng.is_null() {
        return 0;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        rng.next()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_u32(rng: *mut c_void) -> u32 {
    if rng.is_null() {
        return 0;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        rng.next_u32()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_f64(rng: *mut c_void) -> f64 {
    if rng.is_null() {
        return 0.0;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        rng.next_f64()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_f32(rng: *mut c_void) -> f32 {
    if rng.is_null() {
        return 0.0;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        rng.next_f32()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_u64n(rng: *mut c_void, n: u32, out_values: *mut u64) {
    if rng.is_null() || out_values.is_null() {
        return;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        let out_slice = std::slice::from_raw_parts_mut(out_values, n as usize);
        let n = n as usize;
        let mut i = 0;
        while i + 4 < n {
            out_slice[i] = rng.next();
            out_slice[i + 1] = rng.next();
            out_slice[i + 2] = rng.next();
            out_slice[i + 3] = rng.next();
            i += 4;
        }
        for i in i..n {
            out_slice[i] = rng.next();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_u32n(rng: *mut c_void, n: u32, out_values: *mut u32) {
    if rng.is_null() || out_values.is_null() {
        return;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        let out_slice = std::slice::from_raw_parts_mut(out_values, n as usize);
        let n = n as usize;
        let mut i = 0;
        while i + 4 < n {
            out_slice[i] = rng.next_u32();
            out_slice[i + 1] = rng.next_u32();
            out_slice[i + 2] = rng.next_u32();
            out_slice[i + 3] = rng.next_u32();
            i += 4;
        }
        for i in i..n {
            out_slice[i] = rng.next_u32();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_f64n(rng: *mut c_void, n: u32, out_values: *mut f64) {
    if rng.is_null() || out_values.is_null() {
        return;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        let out_slice = std::slice::from_raw_parts_mut(out_values, n as usize);
        let n = n as usize;
        let mut i = 0;
        while i + 4 < n {
            out_slice[i] = rng.next_f64();
            out_slice[i + 1] = rng.next_f64();
            out_slice[i + 2] = rng.next_f64();
            out_slice[i + 3] = rng.next_f64();
            i += 4;
        }
        for i in i..n {
            out_slice[i] = rng.next_f64();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn next_f32n(rng: *mut c_void, n: u32, out_values: *mut f32) {
    if rng.is_null() || out_values.is_null() {
        return;
    }

    unsafe {
        let rng = &mut *(rng as *mut RandGen);
        let out_slice = std::slice::from_raw_parts_mut(out_values, n as usize);
        let n = n as usize;
        let mut i = 0;
        while i + 4 < n {
            out_slice[i] = rng.next_f32();
            out_slice[i + 1] = rng.next_f32();
            out_slice[i + 2] = rng.next_f32();
            out_slice[i + 3] = rng.next_f32();
            i += 4;
        }
        for i in i..n {
            out_slice[i] = rng.next_f32();
        }
    }
}

pub struct RandGen {
    s0: u64,
    s1: u64,
}

impl RandGen {
    pub fn new(in_seed: u64) -> Self {
        RandGen {
            s0: in_seed,
            s1: Self::split_mix_64(in_seed),
        }
    }

    fn split_mix_64(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9e3779b97f4a7c15);
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }

    #[inline(always)]
    fn rotate_left(x: u64, k: u32) -> u64 {
        (x << k) | (x >> (64 - k))
    }

    #[inline(always)]
    pub fn next(&mut self) -> u64 {
        let s0 = self.s0;
        let s1 = self.s1;
        let result = Self::rotate_left(s0.wrapping_add(s1), 17).wrapping_add(s0);

        self.s1 ^= s0;
        self.s0 = Self::rotate_left(s0, 49) ^ self.s1 ^ (self.s1 << 21);
        self.s1 = Self::rotate_left(self.s1, 28);

        result >> 32
    }

    #[inline(always)]
    pub fn next_u32(&mut self) -> u32 {
        (self.next() & 0xFFFFFFFF) as u32
    }

    #[inline(always)]
    pub fn next_f64(&mut self) -> f64 {
        unsafe { *(&((self.next() << 20) | 0x3ff0000000000000) as *const u64 as *const f64) - 1.0 }
    }

    #[inline(always)]
    pub fn next_f32(&mut self) -> f32 {
        unsafe { *(&((self.next_u32() >> 9) | 0x3f800000) as *const u32 as *const f32) - 1.0 }
    }
}
