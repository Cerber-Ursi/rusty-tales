pub fn deliver_to<T: Default>(vec: &mut Vec<T>) {
    vec.extend(vec![Default::default()])
}