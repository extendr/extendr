# `extendr-engine`

This crate is mostly use for testing [`extendr-api`]. See
[`extendr-api`] for more details.

[`extendr-api`]: https://extendr.github.io/extendr/extendr_api/index.html

Provides a singleton instance of the R interpreter.

Only call this from `main()` if you want to run stand-alone.

Its principal use is for testing.

See [`Rembedded.c`](https://github.com/wch/r-source/blob/trunk/src/unix/Rembedded.c) for more details.
