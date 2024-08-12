
## craymath

This repo is a hastily thrown together set of bindings for raymath, with which to test against my port in [niveluno](https://github.com/computermouth/niveluno).

Since neither raylib, nor any downstream port I could find actually has unit tests, and since I found the [same error in both the rust and golang ports](https://github.com/raylib-rs/raylib-rs/pull/73), I've resorted to psuedo-fuzzing, and comparing the results between the C version and my rust version.

This repo doesn't really build properly, I couldn't be bothered to figure out how the build.rs actually works, so the bindings aren't updated on the header change, and it doesn't actually link against the libraymath.so in the repo. Godspeed.