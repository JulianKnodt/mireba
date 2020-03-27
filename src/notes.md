# Various Notes

These are just notes made while composing this library, I don't know if they're that useful.

## Ideas

Is it possible to map 4x4 matrices to octonions and then invert those? Might that be more
computationally efficient than sticking with their own representation?

Wouldn't it be trivial to define vectors as a field with element-wise operations? It just
wouldn't be defined when any element was 0, which is fine because scalars also can't handle
that. In that case, can we define 2D matrices as vector spaces over vectors? I wonder what
implication this would have. What if the norms of vectors were vectors instead of scalars, but
if you really wanted a scalar you could just take the norm of the vector. Interesting to think
about but no idea what the implications would be.

