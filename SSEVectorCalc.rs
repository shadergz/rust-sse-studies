#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[target_feature(enable = "sse2")]
unsafe fn vector_calc(x: &[i32; 4], y: &[i32; 4], z: &[i32; 4]) -> [i32; 4] {
    let x_data = _mm_loadu_si128(x.as_ptr() as *const _);
    let y_data = _mm_loadu_si128(y.as_ptr() as *const _);
    let z_data = _mm_loadu_si128(z.as_ptr() as *const _);

    let xy_item = _mm_add_epi32(x_data, y_data);

    let product = _mm_mullo_epi32(xy_item, z_data);

    let mut result_values: [i32; 4] = [0; 4];

    _mm_storeu_si128(result_values.as_mut_ptr() as *mut _, product);

    result_values
}

fn main() {
    let x_vector: [i32; 4] = [10, -20, 30, 40];
    let y_vector: [i32; 4] = [15, 30, 10, -20];
    let z_vector: [i32; 4] = [5, -20, 70, 20];

    unsafe {
        let result = vector_calc(&x_vector, &y_vector, &z_vector);
        println!("Result: {:?}", result);
    }
}
