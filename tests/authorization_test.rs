use spotifyrs::Spotify;

#[test]
fn client_id_import() {
    let spotify = Spotify::authenticate(
        String::from("8888"),
        String::from("user-read-private user-read-email"),
    );

    let album_info = spotify.get_album_tracks(&"4R09OvFyz47HfjecIjoEtP", Some("US"), Some(1), None).unwrap();

    println!("{:?}", album_info);

    // assert_eq!(
    //     details_object.client_id,
    //     String::from("e02483bb125343f79b9ff4c3c3da74da")
    // )
}

// #[test]
// fn getting_authorization_code() {
//     let details_object = ApplicationDetails::new(
//         String::from("http://localhost:8888/callback"),
//         String::from("user-read-private user-read-email"),
//     );
// }