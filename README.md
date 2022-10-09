# CONTROLS
 * J/K - up/down
 * Enter - select demo
 * Space - show/hide menu
 * R - reset / next
 * Shift click - zoom in (only on some)
 * Ctrl click - zoom out (only on some)

# TODO
 * font rendering
 * clean up, put on gh
 * ponder scene registry further
 * colour based on sized for perco
 * show paths in mandelbrot
 * slider MMB to reset
 * slider scroll for fine
 * fractals are upside down


# DEMO IDEAS:
* rgb noiser
* rgb noise transitions
* percolizer could go all the way down or all the way up to change seed
* PP imagine a matrix of 'what eats what'

## Predator Prey
Just need parameters where more not necessarily == better
 * per parameter mutation rate
 * chance to skip turn: lets you save energy: save next turn as well
 * tiredness: skip but as 
 * give birth threshold: because too low and the kids will die
    * is this identical to number of kids? maybe not, like you could seriously hang on until the good times come
 * x/y of map to inhabit....
 * explorativeness: chance to walk away from where you have been vs go back
 * red and green grass: digestion preference, should be a nice auto niche

 some of these are just for adapting but red and green is definite niche material
 imagine evolving a matrix... thats like the structure of a nn


## Fractals
You can tell that the mandelbrot set, julia set are self referential

possible fractals: things you can do to a Vec2
if mandelbrot is // p such that |(((p^2 + p)^2 + p)^2 + p)^2 + ...| <= 2.0
julia is p such that |(((p2 + p)^2 + p)^2 + ...| <= 2.0
just snuck in a constant at the start
what if we defined a fractal as a dual process of 2 points fighting, ie subtracting
(p1^2 - p2^2)^2 + p1)^2 - p2^2 ...