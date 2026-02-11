# sealbindings

This crate exposes *seal*'s mluau C-Stack API bindings for *seal* FFI plugins.

You should use this crate instead of binding separately to mlua or mluau.

## Usage

At the top of your `seal_open_extern` function, call `sealbindings::initialize(ptr)` with the
ptr to the `LuauApi` struct passed by *seal*.

After doing so, you can safely use the C-stack API methods in `sealbindings::ffi`.
