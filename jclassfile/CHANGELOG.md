# 0.6.1

## What's Changed
* Added crate-level documentation, including when to prefer [`jclassmodel`](https://crates.io/crates/jclassmodel) for a resolved view over the parsed class file. Documentation only; no API or behaviour change.


# 0.6.0

## What's Changed
* Support for `RuntimeInvisibleTypeAnnotations` attribute


# 0.5.0

## What's Changed
* Support for `RuntimeInvisibleParameterAnnotations` and `SourceDebugExtension` attributes (thanks [@exoego](https://github.com/exoego))


# 0.4.0

## What's Changed
* Non-valid CESU-8 sequences are now replaced with '?' instead of raising an error.


# 0.3.0

## What's Changed
* Support for RuntimeVisibleParameterAnnotations attribute.
* Support for RuntimeVisibleTypeAnnotations attribute.
