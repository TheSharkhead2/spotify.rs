mod authorization;
mod spotify;
mod srequest;
mod object_formatting;
mod albums;
mod artists;
mod tracks;

pub use spotify::Spotify; // re-export Spotify struct

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
