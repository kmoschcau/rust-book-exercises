use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(&u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl <T> Cacher<T>
    where T: Fn(&u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &u32) -> u32 {
        match self.values.get(arg) {
            Some(value) => *value,
            None => (self.calculation)(arg)
        }
    }
}

#[cfg(test)]
mod cacher_tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut cacher = Cacher::new(|&a| a);

        let _value1 = cacher.value(&1);
        let value2 = cacher.value(&2);

        assert_eq!(value2, 2);
    }
}
