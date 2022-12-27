pub const SECRET: u64 = 123456789012345678;
pub fn branching_u64(x: u64) {
    let mut _counter = 0;
    for _ in 0..1000 {
        if x == SECRET {
            _counter += 1;
        } else {
            match x {
                x if x < SECRET => _counter += 2,
                x if x > SECRET => _counter -= 2,
                _ => (),
            }
        }
    }
}

pub fn branching_array(x: [u64; 100]) {
    let secret_array: [u64; 100] = (0..100)
        .into_iter()
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();
    let mut _counter = 0;
    for _ in 0..1000 {
        if x == secret_array {
            _counter += 1;
        } else {
            match x {
                x if x[0] < 10 => _counter += 2,
                x if x[0] > 10 => _counter -= 2,
                _ => (),
            }
        }
    }
}
