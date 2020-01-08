# some-graphs-in-rust

## A small program that draws some graphs with n nodes on random positions and random vertices.

A small application I wrote to practice my new Rust knowledge I acquired by reading the [Rust book](https://doc.rust-lang.org/book/title-page.html).

I used the sdl2 crate and the x86_64-pc-windows-msvc toolchain since I do not need to deal with the Microsoft compiler when compiling Rust code. Otherwise I would use the x86_64-pc-windows-gnu or the x86_64-pc-linux-gnu toolchain.

Remember: You can view this by executing the following command:

```
PS C:\> rustc -v --version
rustc 1.40.0 (73528e339 2019-12-16)
binary: rustc
commit-hash: 73528e339aae0f17a15ffa49a8ac608f50c6cf14
commit-date: 2019-12-16
host: x86_64-pc-windows-msvc
release: 1.40.0
LLVM version: 9.0
PS C:\>
```

This should work on any platform.

There are also some notes in the rust-notes file, which are more than trivial but might be useful for other beginners who did not know the ownership system in C++ (which is basically nonexistent, when it comes down to it). Rust on the other hand handles allocated memory beautifully.

## Command line Arguments

```
some-graphs-in-rust <n>
    with n being the number of nodes.
```

Example:

```
PS C:\Users\Valentin\Dev\some-graphs-in-rust\target\release> .\some-graphs-in-rust.exe 10
Drawing graphs with 10 nodes
PS C:\Users\Valentin\Dev\some-graphs-in-rust\target\release> 
```

You can redraw the canvas by pressing any key you like.
