fn matrix_dot_vector(a: Vec<Vec<f64>>, b: Vec<f64>) -> Result<Vec<f64>, i32> {
    // Check if the matrix and vector dimensions are compatible
    if a.is_empty() || a[0].len() != b.len() {
        return Err(-1);  // Return -1 in case of a dimension mismatch
    }

    let mut vals = Vec::new();

    // Perform dot product for each row in the matrix
    for row in a.iter() {
        let mut hold = 0.0;
        for j in 0..row.len() {
            hold += row[j] * b[j];
        }
        vals.push(hold);
    }

    Ok(vals)  // Return the result if everything is fine
}

fn main() {
    // Example usage of the matrix_dot_vector function
    let matrix = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ];
    let vector = vec![1.0, 0.0, -1.0];

    match matrix_dot_vector(matrix, vector) {
        Ok(result) => println!("Dot product result: {:?}", result),
        Err(e) => println!("Error: Dimensions mismatch, code {}", e),
    }
}

