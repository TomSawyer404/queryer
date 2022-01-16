mod convert;
mod dialect;
mod fetcher;

pub use dialect::example_sql;
pub use dialect::TryDialect;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
