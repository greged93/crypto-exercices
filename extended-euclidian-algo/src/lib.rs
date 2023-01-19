pub fn extended_euclian_algorithm(a: u32, b: u32) -> (u32, i32, i32){
    return recurse(a,b,1,0,0,1);
}

fn recurse(a: u32, b:u32, s_a: i32, t_a: i32, s_b: i32, t_b: i32) -> (u32, i32, i32) {
    if a == 0 {
        return (b, s_b, t_b);
    }
    let q = (b/a) as i32;
    recurse(b%a, a, s_b-q*s_a, t_b-q*t_a, s_a, t_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eea_gcd_1() {
        assert_eq!((1, -754, 127), extended_euclian_algorithm(397, 2357));
    }

    #[test]
    fn test_eea_gcd_2() {
        assert_eq!((2, 47, -9), extended_euclian_algorithm(46, 240));
    }
}
