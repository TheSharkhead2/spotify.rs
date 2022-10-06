use spotifyrs::{Spotify, TimeRange};

#[test]
fn general_testing() {
    let mut spotify = Spotify::authenticate(
        String::from("8888"),
        String::from("user-follow-modify playlist-modify-public playlist-modify-private"),
    );
    
    // println!("{:?}", spotify.get_album("1xJ7jIK1tT0aVoJw1fPE6r", None));
    // println!("{:?}", spotify.get_albums(vec!["1xJ7jIK1tT0aVoJw1fPE6r", "1xJ7jIK1tT0aVoJw1fPE6r"], None));
    // println!("{:?}", spotify.get_album_tracks("1xJ7jIK1tT0aVoJw1fPE6r", None, None, None));
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
    // println!("{:?}", spotify.get_tracks_audio_features(vec!["1Hg3GtuEEpPT8NU49xC71Z", "1uviKYHZuM4uINK33F7sCt"]));
    // println!("{:?}", spotify.get_track_audio_features("1Hg3GtuEEpPT8NU49xC71Z"));
    // println!("{:?}", spotify.get_track_audio_analysis("1Hg3GtuEEpPT8NU49xC71Z"));
    // println!("{:?}", spotify.get_recommendations(None, None, Some(vec!["1Hg3GtuEEpPT8NU49xC71Z"]), Some(vec![("target_danceability", "0.8")])));
    // println!("{:?}", spotify.get_current_users_profile());
    // println!("{:?}", spotify.get_users_top_artists(Some(TimeRange::LongTerm), None, None));
    // println!("{:?}", spotify.get_users_top_tracks(Some(TimeRange::ShortTerm), None, None))
    // println!("{:?}", spotify.get_users_profile("kcm4s9xdvua5ft5glrsxii3ki"));
    // println!("{:?}", spotify.follow_playlist("4yNivColKnMGbTe9P3lRjR", Some(true)));
    // println!("{:?}", spotify.unfollow_playlist("4yNivColKnMGbTe9P3lRjR"));
    // println!("{:?}", spotify.get_followed_artists(None));
    // println!("{:?}", spotify.follow_artists(vec!["6eUKZXaKkcviH0Ku9w2n3V", "59sBwR0jPSTrbMtuTkRPN5"]));
    // println!("{:?}", spotify.follow_users(vec!["xk6cplfegqhw6rwezfuvr1198"]));
    // println!("{:?}", spotify.unfollow_artists(vec!["6eUKZXaKkcviH0Ku9w2n3V", "59sBwR0jPSTrbMtuTkRPN5"]));
    // println!("{:?}", spotify.unfollow_users(vec!["xk6cplfegqhw6rwezfuvr1198"]));
    // println!("{:?}", spotify.check_user_follows_artists(vec!["6eUKZXaKkcviH0Ku9w2n3V", "59sBwR0jPSTrbMtuTkRPN5"]));
    // println!("{:?}", spotify.check_user_follows_users(vec!["xk6cplfegqhw6rwezfuvr1198", "kcm4s9xdvua5ft5glrsxii3ki"]));
    // println!("{:?}", spotify.check_users_follow_playlist("4soTsWdI5kIAxa9kACgJb4", vec!["kcm4s9xdvua5ft5glrsxii3ki", "ommmrjvmegv5jpe6cjfc97392", "jazzdancer16"]));
    // println!("{:?}", spotify.get_playlist("1SH1tptnz2C09EndCJb5Zz", None));
    println!("{:?}", spotify.change_playlist_details("40KoEtcGjMKLZJloarHBGw", Some("Craig Johnson"), Some(true), None, None))
}

