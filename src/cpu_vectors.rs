#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::u16;

#[target_feature(enable = "sse2")]
pub unsafe fn vector_calc(x: &[i32; 4], y: &[i32; 4], z: &[i32; 4]) -> [i32; 4] {
    let x_data = _mm_loadu_si128(x.as_ptr() as *const _);
    let y_data = _mm_loadu_si128(y.as_ptr() as *const _);
    let z_data = _mm_loadu_si128(z.as_ptr() as *const _);
    /* Adding all elements from x vector and y vector and storing the result into xy_item */
    let xy_item = _mm_add_epi32(x_data, y_data);

    /* Multiplying result from adding x + y vectors with z vector */
    let product = _mm_mullo_epi32(xy_item, z_data);

    /* Creating results */
    let mut result_values: [i32; 4] = [0; 4];
    /* Storing the vector into a integer array */
    _mm_storeu_si128(result_values.as_mut_ptr() as *mut _, product);

    result_values
}

pub unsafe fn vector_cmp(vector_one: &[i32; 4], vector_two: &[i32; 4]) -> bool {
    /* Loading two vectors from two floats */
    let one_vec = _mm_loadu_si128(vector_one.as_ptr() as *const _);
    let two_vec = _mm_loadu_si128(vector_two.as_ptr() as *const _);
    /* Comparing two vectors */
    let cmp_res = _mm_cmpeq_epi32(one_vec, two_vec);
    /* Storing every MSB of each bit 8 elements inside cmp_res and storing into res */
    let res = _mm_movemask_epi8(cmp_res);
    res as u16 == u16::MAX
}
