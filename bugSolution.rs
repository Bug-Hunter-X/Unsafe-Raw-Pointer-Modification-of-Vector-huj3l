fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    unsafe {
        *v.get_unchecked_mut(index) = 4;
    }
    println!("{:?}", v);
}