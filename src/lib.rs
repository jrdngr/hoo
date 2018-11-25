pub mod api;
pub mod light;
pub mod color;

pub type AnyError = Box<dyn std::error::Error>;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
