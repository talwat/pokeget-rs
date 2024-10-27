# Other Projects

Please note some of these might not be maintained.

## [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts/)

It's changed quite a bit since I last looked at it,
as it is now written in python, and many of my previous issues with it have been fixed.

It actually uses the same sprite database that pokeget does, which is quite cool.
However, it also requires the actual sprite files be on your machine, so it isn't very portable.

## [pokeshell](https://github.com/acxz/pokeshell)

It is written in pure bash, but calls terminal image viewers like [timg](https://github.com/hzeller/timg)/[chafa](https://github.com/hpjansson/chafa) for display so it is slower.

Other than that, it's extremely feature rich with resizing for terminal fit, tab completions, animations and more.

## [krabby](https://github.com/yannjor/krabby)

Krabby is also written in rust, but it isn't very fast and can't display multiple sprites at once.
The slowness is because it uses a full JSON API to retrieve forms and pokemon data, while pokeget
settles with simpler guesses and compromises. As a result, pokeget is much faster than Krabby by a
factor of about ~5x on an M1 Macbook.

Although it has a random shiny option, which in my opinion is absolutely awesome.
