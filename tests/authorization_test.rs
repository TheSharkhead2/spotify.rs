use spotifyrs::Spotify;

#[test]
fn client_id_import() {
    let spotify = Spotify::authenticate(
        String::from("8888"),
        String::from("user-read-private user-read-email"),
    );

    // let album_info = spotify.get_album_tracks(&"4R09OvFyz47HfjecIjoEtP", Some("US"), Some(1), None).unwrap();
    
    // println!("{:?}", spotify.get_album("1xJ7jIK1tT0aVoJw1fPE6r", None));
    // println!("{:?}", spotify.get_saved_albums(None, None, None));
    // println!("{:?}", spotify.save_albums(vec!["1xJ7jIK1tT0aVoJw1fPE6r"]));
    // println!("{:?}", spotify.remove_albums(vec!["1xJ7jIK1tT0aVoJw1fPE6r"]));
    // println!("{:?}", spotify.check_saved_albums(vec!["1xJ7jIK1tT0aVoJw1fPE6r", "4R09OvFyz47HfjecIjoEtP"]));
    // println!("{:?}", spotify.get_new_releases(None, None, None));
    // println!("{:?}", spotify.get_artist("59sBwR0jPSTrbMtuTkRPN5"));
    // println!("{:?}", spotify.get_several_artists(vec!["59sBwR0jPSTrbMtuTkRPN5", "6eUKZXaKkcviH0Ku9w2n3V"]));
    // println!("{:?}", spotify.get_artist_albums("6eUKZXaKkcviH0Ku9w2n3V", None, None, None, None));
    // println!("{:?}", spotify.get_artist_top_tracks("6eUKZXaKkcviH0Ku9w2n3V", "US"));
    // println!("{:?}", spotify.get_artist_related_artists("6eUKZXaKkcviH0Ku9w2n3V"));
    // println!("{:?}", spotify.get_track("212AgAhFl3RJZGAK0LrMpX"));
    // println!("{:?}", spotify.get_several_tracks(vec!["212AgAhFl3RJZGAK0LrMpX","5QYnNhTKsN3kE7OaqILA1U"], None));
    // println!("{:?}", spotify.get_user_saved_tracks(None, None, None));
    // println!("{:?}", spotify.save_tracks(vec!["1Hg3GtuEEpPT8NU49xC71Z", "1uviKYHZuM4uINK33F7sCt"]));
    // println!("{:?}", spotify.remove_tracks(vec!["1Hg3GtuEEpPT8NU49xC71Z", "1uviKYHZuM4uINK33F7sCt"]));
    // println!("{:?}", spotify.check_saved_tracks(vec!["1Hg3GtuEEpPT8NU49xC71Z", "1uviKYHZuM4uINK33F7sCt", "2ZwihAP8zB5XX1CmYIOBbF"]));
    // println!("{:?}", spotify.get_tracks_audio_features(vec!["1Hg3GtuEEpPT8NU49xC71Z", "1uviKYHZuM4uINK33F7sCt"]))
    println!("{:?}", spotify.get_track_audio_features("1Hg3GtuEEpPT8NU49xC71Z"))
    // println!("{:?}", spotify.get_track_audio_analysis("1Hg3GtuEEpPT8NU49xC71Z"));

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