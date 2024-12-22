fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointer, iterate through the vector and modify elements
    for i in 0..v.len() {
        v[i] = v[i] * 2;
    }
    println!("Modified Vector: {:?}", v);
} 