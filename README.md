# pokeget-rs

A better rust version of pokeget.

## Usage

`pokeget <pokemon>`

for more info, run `pokeget --help`

Also, if you're using pokeget in your bashrc, then instead of running `pokeget <pokemon>`,
you can just write the output to a file by doing: `pokeget <pokemon> > file.txt` and then
have something like `cat file.txt` bashrc.

You can also use multiple pokemon like:

`pokeget bulbasaur pikachu`

and dex id's work too:

`pokeget 1 2 3`

## Installation

If you're on arch, you can use the AUR:

```sh
yay -S pokeget
```

You can either use cargo by doing:

```sh
cargo install pokeget
```

and making sure `$HOME/.cargo/bin` is added to `$PATH`.

or clone the repository and compiling manually by doing:

```sh
git clone --recurse-submodules https://github.com/talwat/pokeget-rs.git
cd pokeget-rs
cargo build --release
mv target/release/pokeget ~/.local/bin
```

and making sure `$HOME/.local/bin` is added to `$PATH`.

## Updating

Just rerun `cargo install pokeget` or `git pull` on the repository and then recompile.

### Adding a directory to $PATH

#### Bash & Zsh

Append this to your `.bashrc` or `.zshrc`:

```sh
export PATH="<path>:$PATH"
```

#### Fish

Run this in your CLI:

```sh
fish_add_path <path>
```

## Why?

Because the first pokeget was slow, bloated, and super complicated I decided to make a better version in rust.

Now, instead of precomputing all the sprites and uploading them to a repo, pokeget will
be able to compute them on the fly which makes everything much more flexible while still retaining performance.

It will also draw the sprites 2x smaller by using half squares.

## What about other projects?

pokeget-rs has an edge over projects like the old pokeget, pokeshell, etc... since it's in rust.

It also is significantly (5.5x) faster than krabby which is another very similar project.

For more info, go to [OTHER_PROJECTS.md](OTHER_PROJECTS.md).

## Where are the prebuilt binaries?

I cannot figure out how to compile rust to multiple different platforms with a CI pipeline like github actions.
If someone knows how, PLEASE make a PR.

## What about big sprites?

Gone. Reduced to atoms.

In all seriousness, i've just decided to not deal with them since it's extra work that I don't want to deal with.

## Credits

This time, the sprites are from [pokesprite](https://github.com/msikma/pokesprite) and pokeget uses them with a submodule.

Sprites are embedded into the binary, so pokeget won't download them.
