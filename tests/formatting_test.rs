use json::object;
use spotifyrs::{SpotifyObject, Album, Artist, Track, Playlist};

// I am not sure what these songs are, or if they exist, but Copilot thought they did 
#[test]
fn album_formatting() {
    let album_json = object!{
        "album_type": "album",
        "total_tracks": 10,
        "available_markets": ["CA", "US", "ES"],
        "external_urls": {
            "spotify": "https://open.spotify.com/album/6JWc4iAiJ9FjyK0B59ABb4"
        },
        "href": "https://api.spotify.com/v1/albums/6JWc4iAiJ9FjyK0B59ABb4",
        "id": "6JWc4iAiJ9FjyK0B59ABb4",
        "images": {
            "height": 640,
            "url": "https://i.scdn.co/image/ab67616d0000b273a0e7b8303c2d7c3b2eaa958e",
            "width": 640
        },
        "name": "The Suburbs",
        "release_date": "2010-08-02",
        "release_date_precision": "day",
        "type": "album",
        "uri": "spotify:album:6JWc4iAiJ9FjyK0B59ABb4",
    };

    assert_eq!(Album::new(&album_json).name, "The Suburbs");
}

#[test]
fn artist_formatting() {
    let artist_json = object!{
        "external_urls": {
            "spotify": "https://open.spotify.com/artist/0OdUWJ0sBjDrqHygGUXeCF"
        },
        "followers": {
            "href": null,
            "total": 0
        },
        "genres": [],
        "href": "https://api.spotify.com/v1/artists/0OdUWJ0sBjDrqHygGUXeCF",
        "id": "0OdUWJ0sBjDrqHygGUXeCF",
        "images": [],
        "name": "Arcade Fire",
        "popularity": 0,
        "type": "artist",
        "uri": "spotify:artist:0OdUWJ0sBjDrqHygGUXeCF"
    };

    assert_eq!(Artist::new(&artist_json).name, "Arcade Fire");
}

#[test]
fn track_formatting() {
    let track_json = object!{
        "album": {
            "album_type": "album",
            "artists": [
                {
                    "external_urls": {
                        "spotify": "https://open.spotify.com/artist/0OdUWJ0sBjDrqHygGUXeCF"
                    },
                    "href": "https://api.spotify.com/v1/artists/0OdUWJ0sBjDrqHygGUXeCF",
                    "id": "0OdUWJ0sBjDrqHygGUXeCF",
                    "name": "Arcade Fire",
                    "type": "artist",
                    "uri": "spotify:artist:0OdUWJ0sBjDrqHygGUXeCF"
                }
            ],
            "available_markets": ["CA", "US", "ES"],
            "external_urls": {
                "spotify": "https://open.spotify.com/album/6JWc4iAiJ9FjyK0B59ABb4"
            },
            "href": "https://api.spotify.com/v1/albums/6JWc4iAiJ9FjyK0B59ABb4",
            "id": "6JWc4iAiJ9FjyK0B59ABb4",
            "images": {
                "height": 640,
                "url": "https://i.scdn.co/image/ab67616d0000b273a0e7b8303c2d7c3b2eaa958e",
                "width": 640
            },
            "name": "The Suburbs",
            "release_date": "2010-08-02",
            "release_date_precision": "day",
            "type": "album",
            "uri": "spotify:album:6JWc4iAiJ9FjyK0B59ABb4",
        },
        "artists": [
            {
                "external_urls": {
                    "spotify": "https://open.spotify.com/artist/0OdUWJ0sBjDrqHygGUXeCF"
                },
                "href": "https://api.spotify.com/v1/artists/0OdUWJ0sBjDrqHygGUXeCF",
                "id": "0OdUWJ0sBjDrqHygGUXeCF",
                "name": "Arcade Fire",
                "type": "artist",
                "uri": "spotify:artist:0OdUWJ0sBjDrqHygGUXeCF"
            }],
        "available_markets": ["CA", "US", "ES"],
        "disc_number": 1,
        "duration_ms": 238373,
        "explicit": false,
        "external_ids": {
            "isrc": "USWB11000689"
        },
        "external_urls": {
            "spotify": "https://open.spotify.com/track/6JWc4iAiJ9FjyK0B59ABb4"
        },
        "href": "https://api.spotify.com/v1/tracks/6JWc4iAiJ9FjyK0B59ABb4",
        "id": "6JWc4iAiJ9FjyK0B59ABb4",
        "is_local": false,
        "name": "The Suburbs",
        "popularity": 0,
        "preview_url": "https://p.scdn.co/mp3-preview/ab67616d0000b273a0e7b8303c2d7c3b2eaa958e?cid=774b29d4f13844c495f206cafdad9c86",
        "track_number": 1,
        "type": "track",
        "uri": "spotify:track:6JWc4iAiJ9FjyK0B59ABb4",
    };

    assert_eq!(Track::new(&track_json).name, "The Suburbs");
    assert_eq!(Track::new(&track_json).album.unwrap().name, "The Suburbs");
}

#[test]
fn playlist_formatting() {
    let playlist_json = object!{
        "collaborative": false,
        "description": "A playlist featuring Arcade Fire",
        "external_urls": {
            "spotify": "https://open.spotify.com/playlist/37i9dQZF1DX0XUsuxWHRQd"
        },
        "followers": {
            "href": null,
            "total": 0
        },
        "href": "https://api.spotify.com/v1/playlists/37i9dQZF1DX0XUsuxWHRQd",
        "id": "37i9dQZF1DX0XUsuxWHRQd",
        "images": [
            {
                "height": 300,
                "url": "https://i.scdn.co/image/ab67706f00000003a0e7b8303c2d7c3b2eaa958e",
                "width": 300
            }
        ],
        "name": "Arcade Fire",
        "owner": {
            "display_name": "Spotify",
            "external_urls": {
                "spotify": "https://open.spotify.com/user/spotify"
            },
            "href": "https://api.spotify.com/v1/users/spotify",
            "id": "spotify",
            "type": "user",
            "uri": "spotify:user:spotify"
        },
        "primary_color": null,
        "public": true,
        "snapshot_id": "MTYwMjQ0MjYwMCwwMDAwMDAwMDAwMDAwMTc5ZjY3ZjY0ZjYwMDAwMDE3ZjY3ZjY0ZjYw",
        "tracks": {
            "href": "https://api.spotify.com/v1/playlists/37i9dQZF1DX0XUsuxWHRQd/tracks",
            "total": 0
        },
        "type": "playlist",
        "uri": "spotify:playlist:37i9dQZF1DX0XUsuxWHRQd"
    };

    assert_eq!(Playlist::new(&playlist_json).name, "Arcade Fire");
}