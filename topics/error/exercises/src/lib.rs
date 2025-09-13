#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
    Other
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    let z: Result<u32, MathError> = if y != 0 {
        Ok(x / y)
    } else {
        Err(MathError::DivByZero)
    };
    return z;
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    match v.get(i) {
        Some(&val) => val,
        None => default_val,
    }
}
