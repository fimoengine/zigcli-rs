extern "C" {
    #[allow(unused)]
    fn add(left: i32, right: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = unsafe { add(2, 2) };
        assert_eq!(result, 4);
    }
}
