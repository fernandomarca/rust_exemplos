fn add<T: Into<Option<u32>>>(a: u32, b: T) -> u32 {
    if let Some(b) = b.into() {
        a + b
    } else {
        a
    }
}

fn main() {
    assert_eq!(add(3, 4), 7);
    assert_eq!(add(8, None), 8);
}
