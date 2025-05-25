use std::ffi::c_void;
use std::time::Instant;

#[unsafe(no_mangle)]
pub extern "C" fn new_vec_in_stack(v: *mut c_void) {
    unsafe {
        std::ptr::write(v as *mut Vec<u64>, vec![]);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_vec_in_stack(v: *mut c_void) {
    unsafe {
        _ = std::ptr::read(v as *mut Vec<u64>);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn push_to_vec(v: *mut c_void, i: u64) {
    let v = unsafe { &mut *(v as *mut Vec<u64>) };
    v.push(i);
}

unsafe extern "C" {
    fn do_the_job();
}

fn build_vec(n: u64) -> Vec<u64> {
    let mut r = vec![];
    for i in 0..n {
        r.push(i);
    }
    r
}

fn main() {
    let start = Instant::now();
    for _ in 0..100_000 {
        std::hint::black_box(build_vec(10000));
    }
    println!("Pure rust = {:?}", start.elapsed());

    let start = Instant::now();
    unsafe {
        do_the_job();
    }
    println!("Cross language = {:?}", start.elapsed());
}
