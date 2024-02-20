use spotifyrs::{AlbumId, Error};

#[test]
fn album_id_constructs_properly() -> Result<(), Error> {
    // create album id from an id
    let album_id_from_id: AlbumId = AlbumId::from_id("2up3OPMp9Tb4dAKM2erWXQ");

    // create same album id but using URI
    let album_id_from_uri: AlbumId = AlbumId::from_uri("spotify:album:2up3OPMp9Tb4dAKM2erWXQ")?;

    // create same album id still, but with url
    let album_id_from_url: AlbumId =
        AlbumId::from_url("http://open.spotify.com/album/2up3OPMp9Tb4dAKM2erWXQ")?;

    // make sure they are actually the same
    assert_eq!(album_id_from_id.id(), album_id_from_uri.id());
    assert_eq!(album_id_from_id.id(), album_id_from_url.id());

    Ok(())
}
