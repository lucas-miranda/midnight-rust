pub mod base;
pub mod components;
pub mod ecs;
pub mod math;
pub mod time;
pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
