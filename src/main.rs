use rumpy::Array;

fn main() {
    let shape = vec![2, 3, 4];
    let data = vec![1.0; 24];
    let array = Array::from_vec(data, shape).unwrap();

    println!("{:?}", array);
}
