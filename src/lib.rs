mod authorization;
mod spotify;
mod srequest;
mod albums;
mod object_formatting;

pub use spotify::Spotify; // re-export Spotify struct

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
