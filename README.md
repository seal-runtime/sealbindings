# sealbindings

This crate exposes *seal*'s mluau C-Stack API bindings for *seal* FFI plugins.

You should use this crate instead of binding separately to mlua or mluau.

## Usage

At the top of your `seal_open_extern` function, call `sealbindings::initialize()`.
This dynamically loads a function from the `seal` executable, exposing the entire mluau C-stack API.

After doing so, you can safely use the C-stack API methods in `sealbindings::ffi`.
