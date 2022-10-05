mod authorization;
mod spotify;
mod srequest;
mod object_formatting;
mod albums;
mod artists;
mod tracks;
mod users;

pub use spotify::Spotify; // re-export Spotify struct
pub use spotify::TimeRange; // re-export TimeRange enum

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
