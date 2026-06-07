# Chip-8 Emulator

<img width="752" height="784" alt="ibm_logo" src="https://github.com/user-attachments/assets/8ba51f29-20c3-4d2c-998a-f4db10742b15" />

## Development

Development instructions for running the emulator locally

### MacOS

Project dependencies are defined in the `Brewfile` located in the root of the repo.
To install the dependencies, simply run:

```
brew bundle install
```

If you run into linker errors when running or building with cargo, such as:

```
= note: ld: library 'SDL2' not found
        clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

Add the following to your shell config (e.g. `~/.zshrc`) to tell the linker where to look for the sdl2 library, assuming
you installed sdl2 using Homebrew.

```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```
