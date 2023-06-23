# EM-dash

This is my future personal studying program. It should be standalone and capable
of doing backups. At this point, it's a work in progress. It won't be
minimalistic because I really don't want to install several complementary
programs when I'm using this in another computer but I probably will make it
more modular so you can choose which features you want.


## Planned features

- Git support
- LaTeX support
- Better look (sigh)

## Installation

To install the server, run this code:

``` sh
cargo build --release
cd files
mkdir scripts styles templates
touch styles/style.css scripts/script.js
cd ..
```

Now to serve the code, run this:

```sh
cargo run
```

or 

```sh
target/release/em_dash
```

I know, these instructions are terrible and the installation is too but I will improve.
