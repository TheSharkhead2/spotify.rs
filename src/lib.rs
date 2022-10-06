mod authorization;
mod spotify;
mod srequest;
mod object_formatting;
mod albums;
mod artists;
mod tracks;
mod users;
mod playlist;

pub use spotify::{Spotify, SpotifyImage, AlbumType, RestrictionReason, ReleaseDatePrecision, ExternalTrackIds, Album, DatedAlbum, Artist, Track, DatedTrack, FeatureTrack, Bar, Beat, Section, Segment, Tatum, AnalysisTrack, User, TimeRange, Playlist, PlaylistTrack, SpotifyCollection}; // re-export relevant structs and enums

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
