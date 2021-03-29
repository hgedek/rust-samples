// i8 i16 i32 i64 i128
// u8 u16 u32 u64 
// f32 f64
fn reverse(pair: (i32, bool)) -> (bool,i32) {
    let (integer, boolean) = pair;
    return (boolean, integer);
}

#[derive(Debug)]
struct Matrix (f32, f32, f32, f32); 

fn transpose(matrix: Matrix) -> Matrix {
    let m = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    return m;
}

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of array: {}", slice[0]);
    println!("size of array: {}", slice.len());
}

fn main() {
    // formatted printing
    println!("1 + 2 = {}", 1u32 + 2);
    // 0x: hex
    // 0o: oct 
    // 0b: binary 
    println!("hex: {}  oct: {} binary: {}", 0x8032u32, 0o2323u32, 0b101011101);
    println!("long number: {}", 1_000_000_000);

    // tuple
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (-1i8, -2i16, -3i32));
    // printable 
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair : {:?}", pair);
    println!("reverse of pair: {:?}", reverse(pair));

    let matrix = Matrix(1.1, 2.1, 3.1, 4.1);
    println!("Matrix : {:?}", matrix);
    println!("Tranpose: {:?}", transpose(matrix));

    let xs: [i32; 5] = [1,2,3,4,5];
    let os = [1,2,3,4,5];
    
    // [type; lenght] = [value; length]
    let fs: [i32; 10] = [0;10];
    println!("first element of array: {}", xs[0]);
    println!("lenght of the array : {}", xs.len());

    println!("mem size in bytes : {}", mem::size_of_val(&xs));
    println!("mem siz in bytes: {}", mem::size_of_val(&os));

    analyze_slice(&xs);
    analyze_slice(&os[0..4]);

    println!("END");

}
