# Ray

Implicit Surfaces should be simple enough to implement in some cases
so that should be the next thing.

# Splines

Need to handle implementing iterators, because the lifetimes are killing me when dealing with
sampling. This is because the weights are only temporarily created, but it would be much nicer
if they could be statically allocated. This is hard because I've written it with excessive
amounts of generic parameters. Thus, I ended up moving most things into the function, but they
are for the most part copy so it's not too painful.

Also need to add some testing for them to see if they work in 2D and then try it later in 3D.
