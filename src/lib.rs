pub mod animation;
pub mod api;
pub mod color;
pub mod effects;
pub mod light;

pub type AnyError = Box<dyn std::error::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
