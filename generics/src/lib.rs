use generic_array::{typenum::U10, ArrayLength, GenericArray};

struct GenArrayWrapper<T, N: ArrayLength<T>> {
    inner: GenericArray<T, N>,
}

impl<T: Default + PartialEq, N: ArrayLength<T>> GenArrayWrapper<T, N> {
    pub fn new() -> Self {
        GenArrayWrapper::<T, N> {
            inner: GenericArray::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get_index(&self, elem: T) -> usize {
        self.inner
            .iter()
            .enumerate()
            .find(|(_, x)| x == &&elem)
            .unwrap()
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut gen_arr = GenArrayWrapper::<i32, U10>::new();
        gen_arr.inner[0] = 99;
        assert_eq!(99, gen_arr.inner[0])
    }

    #[test]
    fn test_len() {
        let gen_arr = GenArrayWrapper::<i32, U10>::new();
        assert_eq!(10, gen_arr.len())
    }

    #[test]
    fn test_get_index() {
        let mut gen_arr = GenArrayWrapper::<i32, U10>::new();
        gen_arr.inner[0] = 12;
        gen_arr.inner[1] = 13;
        gen_arr.inner[2] = 14;
        gen_arr.inner[3] = 15;
        gen_arr.inner[4] = 16;
        gen_arr.inner[5] = 17;
        gen_arr.inner[6] = 18;
        gen_arr.inner[7] = 19;
        gen_arr.inner[8] = 20;
        gen_arr.inner[9] = 21;
        assert_eq!(7, gen_arr.get_index(19));
    }
}
