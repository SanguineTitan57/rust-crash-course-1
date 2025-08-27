pub fn zeros() -> [u32; 100] {
    let arr: [u32; 100] = [0; 100];
    return arr;
}

pub fn first_3(s: &[u32]) -> &[u32] {
    let slice: &[u32] = &s[..3];
    return slice;
}

pub fn last_3(s: &[u32]) -> &[u32] {
    let last_slice: &[u32] = &s[s.len() - 3..];
    last_slice
}
