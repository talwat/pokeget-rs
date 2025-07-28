## Update Sprites
Need to include Gen 9 sprites.
They can be found at this git repo: https://github.com/cwalke6/pokeget-rs.git

I think the best way to go about this would be to make a fork of the submodule that pokeget-rs repo uses and then in that fork add the sprites from the new file.

## Update Code to Use New Sprites
Need to go through all the rust files in src.
Need to change the region option.
Need to make sure the hisuian pokemon can use the flag and display properly.
Basically update all things in code
Will also need to update female specific pokemon.
Maybe shiny? Not sure how they handle shiny in the rust code.

## Odd Cases to Remember
Genders
Maushold family of four or three
Squawkabilly colors
Palafin - Zero Form/Hero Form
Tatsugiri forms
Dudunsparce - Two/Three-segment Forms
Gimmighoul - Chest/Roaming form
Ogerpon - Different Masks
Terapagos - Normal/Terastal Forms

Not sure if forms are really necessary. If we dont have the sprite then we can't do anything.
If there are no forms in the code then probably leave it as well.

## Update data dir
Probably the easiest here and probably can be scripted. Just have to add the pokemon in order of names to 'pokemon.txt' and 'names.csv'

Scripting actually doesnt really make sese because I would have to type the code + All of the names of the pokemon.
At that point I might as well just type the names of the pokemon into the file. If I already had them or could scrape them somehow then I would but typing manually is probably easier
