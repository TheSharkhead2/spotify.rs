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

    // create same album id but with slightly different url
    let album_id_from_url_other: AlbumId =
        AlbumId::from_url("open.spotify.com/album/2up3OPMp9Tb4dAKM2erWXQ")?;

    // create same album id but with other slightly different url
    let album_id_from_url_other2: AlbumId =
        AlbumId::from_url("https://open.spotify.com/album/2up3OPMp9Tb4dAKM2erWXQ")?;

    // make sure they are actually the same
    assert_eq!(album_id_from_id.id(), album_id_from_uri.id());
    assert_eq!(album_id_from_id.id(), album_id_from_url.id());
    assert_eq!(album_id_from_id.id(), album_id_from_url_other.id());
    assert_eq!(album_id_from_id.id(), album_id_from_url_other2.id());

    Ok(())
}

#[test]
#[should_panic]
fn album_id_cant_construct_other_uri() {
    // create album id from track uri sould fail
    AlbumId::from_uri("spotify:track:6rqhFgbbKwnb9MLmUQDhG6").unwrap();
}

#[test]
#[should_panic]
fn album_id_cant_construct_other_url() {
    // create album id from track url sould fail
    AlbumId::from_uri("http://open.spotify.com/track/6rqhFgbbKwnb9MLmUQDhG6").unwrap();
}

#[test]
fn album_id_cant_construct_invalid_uri() {
    // uri without "spotify" is invalid
    if let Ok(_) = AlbumId::from_uri("album:2up3OPMp9Tb4dAKM2erWXQ") {
        // allowed construction, so fail
        panic!("Constructed invalid AlbumId from URI: 'album:2up3OPMp9Tb4dAKM2erWXQ'");
    }

    // uri without "spotify" is invalid
    if let Ok(_) = AlbumId::from_uri("asdf:album:2up3OPMp9Tb4dAKM2erWXQ") {
        // allowed construction, so fail
        panic!("Constructed invalid AlbumId from URI: 'asdf:album:2up3OPMp9Tb4dAKM2erWXQ'");
    }

    // uri needs colons
    if let Ok(_) = AlbumId::from_uri("2up3OPMp9Tb4dAKM2erWXQ") {
        panic!("Constructed invalid AlbumId from URI: '2up3OPMp9Tb4dAKM2erWXQ'");
    }

    // uri without id is invalid
    if let Ok(_) = AlbumId::from_uri("spotify:album:") {
        // allowed construction, so fail
        panic!("Constructed invalid AlbumId from URI: 'spotify:album:'");
    }
    if let Ok(_) = AlbumId::from_uri("spotify:album") {
        // allowed construction, so fail
        panic!("Constructed invalid AlbumId from URI: 'spotify:album'");
    }
}

#[test]
fn album_id_cant_construct_invalid_url() {
    // url without "spotify" is invalid
    if let Ok(_) = AlbumId::from_url("http://google.com/album/2up3OPMp9Tb4dAKM2erWXQ") {
        // allowed construction, so fail
        panic!("Constructed invalid AlbumId from URL: 'http://google.com/album/2up3OPMp9Tb4dAKM2erWXQ'");
    }

    // url needs to be url
    if let Ok(_) = AlbumId::from_url("asdf:album:2up3OPMp9Tb4dAKM2erWXQ") {
        // allowed construction, so fail
        panic!("Constructed invalid AlbumId from URL: 'asdf:album:2up3OPMp9Tb4dAKM2erWXQ'");
    }

    // shouldn't construct from url without id
    if let Ok(_) = AlbumId::from_url("http://open.spotify.com/album/") {
        panic!("Constructed invalid AlbumId from URL: 'http://open.spotify.com/album/'");
    }
}
