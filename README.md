# Spotifyrs
spotifyrs aims to be a relatively easy to use wrapper for the Spotify API. It is still currently under development (see [issues and roadmap](#-Issues-and-Roadmap)); however, it can be used for most purposes in its current state. 
This wrapper allows you to interact with the Spotify API using simple method calls and consistent objects that give you a consistent way to access the data returned by the API. 

See the [documentation](www.docs.rs/spotifyrs) for a quick start guide. 

If you see any issues with this crate, please [submit an issue](https://github.com/TheSharkhead2/spotify.rs/issues) or make a pull request! If you submit an issue I will try to get to it as soon as possible, this is still an early version, so not all the problems have been worked out. 

## Issues and Roadmap
- Support for saving access tokens and refresh tokens in a file such that the user wouldn't have to reauthenticate every time they open an application
- Support for Episodes, Shows, Audiobooks, and Chapters 
    - I have yet to implement these endpoints because they are features in Spotify which I don't use. Though, I plan on implementing them in the future. In particular, I plan to implement a more thorough implementation for handling the possibility that both episodes and tracks are returned from the "recently listened" endpoint, for example. 
- Support for other authorization code flows. I never implemented other ones as it wasn't required for my purposes (yet); however, as it represents part of the API, I plan to add this in the future. 
    - This also comes with support for other forms of redirect uri outside of just localhost
- Currently, there is poor support if the user quits out of an authorization request. As this is something which is important to handle properly, this is something that I plan on fixing soon. 

## Contributing 
Please do! This started as a passion project for me so that I could build an app that required the Spotify API and also improve my understanding of how the API works; however, I would love it if other people wanted to contribute!