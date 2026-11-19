enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
