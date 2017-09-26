LANGGEN or BILBO BABBITT BABBLES
================================

LANGGEN is my fun project to learn a [RUST](https://www.rust-lang.org).   It processes text files in given language (tested on English) and creates text model using [trigrams](https://en.wikipedia.org/wiki/Trigram). From these trigrams it generates random sentences, which are nonsence, but somehow remind "real" language.

How to use
==========

1. Ensure you have latest RUST environment and cargo.
2. Clone project from Github
3. Get some text files to use as language corpus - for instance this file [all Doyle's Sherlock books in one txt](https://sherlock-holm.es/stories/plain-text/cano.txt) or repository contains script to download top 100 english books from [Project Guttenberg](https://www.gutenberg.org/).
4. build and run http server `cargo run --bin serve --release -- [your_text_files]`

Also Dockerfiles are available in deploy(generic image) and deploy-s2i(builder image for Openshift)

License
=======
MIT or Apache 2.0