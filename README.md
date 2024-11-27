# aterm
Website -  [yew](https://github.com/yewstack/yew) frontend (spa),  [Rocket](https://rocket.rs/) backend.  

To run yew app standalone

```
% cd frontend
% trunk serve --open
```

To serve the app with Rocket: 
```
% cd frontend
% trunk build --release

% cd ../server_rocket
% cargo run --release
```
