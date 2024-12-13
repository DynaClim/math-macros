# Mathmacros

Simple crate providing macros for basic mathemtical operations (min, max, log, trig functions, etc).
Using these macros _may_ assist in the readability of formulas.

```
    (7 + 3 / 2).min(5).min(13)

    min!((7 + 3 / 2), 5, 13)
```

Or, not..

```
    (7. + 3. / 2_f64)
    .sqrt()
    .abs()
    .ln()

    ln!(
        abs!(
            sqrt!(7. + 3. / 2_f64)
            )
        )
    )
```

