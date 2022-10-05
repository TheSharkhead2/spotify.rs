mod authorization;
mod spotify;
mod srequest;
mod object_formatting;
mod albums;
mod artists;
mod tracks;
mod users;

pub use spotify::{Spotify, SpotifyImage, AlbumType, RestrictionReason, ReleaseDatePrecision, ExternalTrackIds, Album, DatedAlbum, Albums, DatedAlbums, Artist, Artists, Track, DatedTrack, Tracks, DatedTracks, FeatureTrack, Bar, Beat, Section, Segment, Tatum, AnalysisTrack, User, TimeRange}; // re-export relevant structs and enums

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
