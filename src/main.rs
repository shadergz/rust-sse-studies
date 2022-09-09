mod cpu_vectors;

fn main() {
    let x_vector: [i32; 4] = [10, -20, 30, 40];
    let y_vector: [i32; 4] = [15, 30, 10, -20];
    let z_vector: [i32; 4] = [5, -20, 70, 20];

    let result_vector: [i32; 4] = [125, -200, 2800, 400];

    unsafe {
        let result = cpu_vectors::vector_calc(&x_vector, &y_vector, &z_vector);
        assert_eq!(cpu_vectors::vector_cmp(&result, &result_vector), true);
        println!("Result: {:?}", result);
    }
}
