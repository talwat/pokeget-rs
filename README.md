# pokeget-rs

A better rust version of pokeget.

## Usage

`pokeget <pokemon>`

For more info, run `pokeget --help`.

### .bashrc

If you're using pokeget on shell startup, such as in `.bashrc`,
then instead of running `pokeget <pokemon>`, you can write the output
to a file by doing: `pokeget <pokemon> > file.txt`
and then have something like `cat file.txt` in your bashrc.

This makes your shell initialization practically instant, but obviously
won't work with random pokemon. pokeget is already fairly fast,
so using it on shell initialization is also not a very large bottleneck.

### Examples

#### Using multiple pokemon

`pokeget bulbasaur pikachu random`

#### Using pokedex ID's

`pokeget 1 2 3`

#### Using alternative forms

`pokeget raichu sandslash meowth --alolan`

#### Using regions

`pokeget kanto`

## Installation

### Cargo *(recommended)*

The recommended installation method is to use cargo:

```sh
cargo install pokeget
```

and making sure `$HOME/.cargo/bin` is added to `$PATH`.

### AUR

If you're on Arch, you can also use the AUR:

```sh
yay -S pokeget
```

### Git

You can also clone the repository and compile manually by doing:

```sh
git clone --recurse-submodules https://github.com/talwat/pokeget-rs.git
cd pokeget-rs
cargo build --release
mv target/release/pokeget ~/.local/bin
```

and making sure `$HOME/.local/bin` is added to `$PATH`.

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

## Updating

Just rerun `cargo install pokeget` or `git pull` on the repository and then recompile.

## Why?

Because the first pokeget was slow, bloated, and super complicated, so I decided to make a better version in rust.

Now, instead of precomputing all the sprites and uploading them to a repo, pokeget will
be able to compute them on-demand which makes everything much more flexible.
Rust enables that computation to be done much more quickly than something like python.

It will also draw the sprites 2x smaller by using half squares.

## What about other projects?

pokeget-rs has an edge over projects like the old pokeget, pokeshell, etc... since it's in rust.

It also is significantly (5.5x) faster than krabby which is another very similar project.

For more info, go to [OTHER_PROJECTS.md](OTHER_PROJECTS.md).

## What about big sprites?

Gone. Reduced to atoms.

In all seriousness, I've just decided to not deal with them since it's significantly
extra work that I don't want to deal with. They were rarely used, and looked ugly
in small terminal windows, so there was little use in keeping them.

## Credits

This time, the sprites are from a forked repo of [pokesprite](https://github.com/cwalke6/pokesprite) and pokeget uses them with a git submodule.

The original repo of pokesprite from msikma can be found [here](https://github.com/msikma/pokesprite)

The Gen 9 sprites that were added were taken from bamq's [repository](https://github.com/bamq/pokemon-sprites) of pokemon sprites.

Sprites are embedded into the binary, so pokeget won't download them. This is a good compromise,
since while the binary may be large, pokeget can execute almost instantly and while offline.

## Palden Sprites

Here lists the work done and credit of original sprites for Paldea.
Simply taking the prexisting sprites work from bamq [here](https://github.com/bamq/pokemon-sprites) and forking miskma's pokesprite repo [here](https://github.com/msikma/pokesprite).

Then updating the submodule to have my forked repo of pokesprite.

Then updating names.csv and pokemon.txt in order to have the names there for it to grab them.
Then some small changes in the rust to add Paldea as a Region as well as change some values to hold.

The only "problem" or thing to note for the future is that the gen9 sprites are still under pokemon-gen8 since I figured it would be easiest to just copy them there and not mess with any of the naming schematics.

Thanks to CalOtter for logical help with this project.

## License

pokeget uses the MIT license, so feel free to fork it and customize it as you please.
If you're unsure about any of the internal workings of pokeget, [open an issue](https://github.com/talwat/pokeget-rs/issues),
and I'll answer whatever question you might have.
