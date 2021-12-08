//pub mod coin;
pub mod cosmos;
pub mod protos;
pub mod tendermint;

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
