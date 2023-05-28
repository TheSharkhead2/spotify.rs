use tokio_test;

use spotifyrs::requesturl_authorization_code;
use spotifyrs::Spotify;
// use spotifyrs::TimeRange;
// use spotifyrs::SpotifyContext;
// use spotifyrs::RepeatState;

// This is really only a testing file to develop functions and make sure they return expected outputs. Hard to test properly
#[test]
fn general_testing() {
    let scope = String::from("user-modify-playback-state");
    let redirect_uri = "http://localhost:8888/callback";
    let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

    // let spotify = Spotify::new(); // create spotify object
    // tokio_test::block_on(spotify.authenticate(String::from("8888"), scope)).unwrap();

    // file saving
    let spotify = tokio_test::block_on(Spotify::new_from_file(".saved_credentials")).unwrap();

    // testing of less restrictive method
    // let (auth_url, state, code_verifier) =
    //     requesturl_authorization_code(&client_id[..], redirect_uri, &scope[..]);

    // println!("{:?}", auth_url);
    // println!("{:?}", state);
    // println!("{:?}", code_verifier);

    // let auth_code = "AQDF3LVUjJOmNxckhoj5O8oq9BgJOo0iqpV5GP_1wWWNWrzWP8oFK_yJpXPdvjoG3RcDRg5oWQrfvko3zJqIs93qwAGq1GShEWzn2YpOtQj5dMEyWmOEF7bGFCqKwE4oDro5yimZNakbKaUGd8PFDg_iuw2C5v-JHNQ3vq1vj8aZikZygYACVkHM1czs7dUXk_YczBU1nfi4bScFSIjhkvGBePR8YBGiQAJAjHHXN36HW20B_3lh0rtgd_OGNgEM2zYC4S1B73yBaOfHB3AST1g";
    // let code_verifier = "3HuxuXebuDzzkzi8hf_sUhhS6Fyz-tfrSn-ePYy6Kzw";

    // let spotify = tokio_test::block_on(Spotify::new_from_auth_code(
    //     auth_code,
    //     &client_id[..],
    //     scope,
    //     code_verifier,
    //     redirect_uri,
    // ));

    println!(
        "{:?}",
        tokio_test::block_on(spotify.get_album("1xJ7jIK1tT0aVoJw1fPE6r", None))
    );
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
    // println!("{:?}", spotify.change_playlist_details("40KoEtcGjMKLZJloarHBGw", Some("Craig Johnson"), Some(true), None, None));
    // println!("{:?}", spotify.get_playlist_tracks("40KoEtcGjMKLZJloarHBGw", None, Some(100), None));
    // println!("{:?}", spotify.add_tracks_to_playlist("1SH1tptnz2C09EndCJb5Zz", vec!["212AgAhFl3RJZGAK0LrMpX","5QYnNhTKsN3kE7OaqILA1U"], None));
    // println!("{:?}", spotify.replace_playlist_tracks("40KoEtcGjMKLZJloarHBGw", vec!["212AgAhFl3RJZGAK0LrMpX","5QYnNhTKsN3kE7OaqILA1U"]));
    // println!("{:?}", spotify.reorder_playlist_tracks("40KoEtcGjMKLZJloarHBGw", 1, 0, None, None));
    // println!("{:?}", spotify.remove_playlist_tracks("40KoEtcGjMKLZJloarHBGw", vec!["212AgAhFl3RJZGAK0LrMpX"], None));
    // println!("{:?}", spotify.get_current_users_playlists(None, None));
    // println!("{:?}", spotify.get_users_playlists("kcm4s9xdvua5ft5glrsxii3ki", None, None));
    // println!("{:?}", spotify.create_playlist("ommmrjvmegv5jpe6cjfc97392", "I made playlist", None, None, Some("I made this playlist with the spotify api")));
    // println!("{:?}", spotify.get_featured_playlists(None, None, None, None, None));
    // println!("{:?}", spotify.get_categorys_playlists("hiphop", None, None, None));
    // println!("{:?}", spotify.get_several_browse_categories(None, None, None, None));
    // println!("{:?}", spotify.get_playlist_cover_image("3rplsOUSIqcwlCV1yHX5f7"));
    // println!("{:?}", spotify.get_single_browse_category("hiphop", None, None));
    // println!("{:?}", spotify.get_available_genre_seeds());
    // println!("{:?}", spotify.get_available_markets());
    // println!("{:?}", spotify.get_playback_state(None));
    // println!("{:?}", spotify.transfer_playback("", false));
    // println!("{:?}", spotify.get_available_devices());
    // println!("{:?}", spotify.get_currently_playing_track(None));
    // println!("{:?}", spotify.start_resume_playback(None, Some(SpotifyContext::Album(String::from("1xJ7jIK1tT0aVoJw1fPE6r"))), None, None, Some("4j9TBVRJVzEPG6wjALFyMt"), Some(8753)));
    // println!("{:?}", spotify.pause_playback(None));
    // println!("{:?}", spotify.skip_next(None));
    // println!("{:?}", spotify.skip_previous(None));
    // println!("{:?}", spotify.seek_position(1234, None));
    // println!("{:?}", spotify.set_repeat_mode(RepeatState::Context, None));
    // println!("{:?}", spotify.set_playback_volume(1, None));
    // println!("{:?}", spotify.toggle_shuffle(true, None));
    // println!("{:?}", spotify.get_recently_played_tracks(None, None, None));
    // println!("{:?}", spotify.get_users_queue());
    // println!("{:?}", spotify.add_track_to_queue("212AgAhFl3RJZGAK0LrMpX", None));

    println!("{:?}", spotify.save_to_file(".saved_credentials"));
}
