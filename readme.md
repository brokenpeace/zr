### z :rat:

<img src="zrat.png" alt="zrat" title="zrat" align="right" width=200 />

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://choosealicense.com/licenses/mpl-2.0/)
[![Build Status](https://travis-ci.org/jedahan/zr.svg?branch=master)](https://travis-ci.org/jedahan/zr)

Quick, simple zsh plugin manager

    zr 0.6.3
    Jonathan Dahan <hi@jonathan.is>
    z:rat: - zsh plugin manager

    USAGE:
        zr [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        help      Prints this message or the help of the given subcommand(s)
        list      list plugins
        load      generate init file from plugin list
        update    update plugins

#### install

You can use crates.io and cargo to just `cargo install zr`

#### usage

Add this to your *~/.zshrc*:

```zsh
# Generate new ~/.zr/init.zsh whenever ~/.zshrc is modified
[[ ~/.zshrc -nt ~/.zr/init.zsh ]] && {
  zr load \
    frmendes/geometry \
    jedahan/geometry-hydrate \
    junegunn/fzf/shell/key-bindings.zsh  # just load key-bindings.zsh
}

source ~/.zr/init.zsh
```

If you'd like a different directory for `~/.zr`, just set `ZR_HOME`

#### speed

The following __[init.zsh][]__ takes 180ms total, with 20ms spent in `zr load` for my 2012 13" retina macbook pro.
See [the wiki](https://github.com/jedahan/zr/wiki) for more details.

    # ~20ms
    zr load \
      zsh-users/prezto/modules/git/alias.zsh \
      zsh-users/prezto/modules/osx/init.zsh \
      zsh-users/prezto/modules/history/init.zsh \
      junegunn/fzf/shell/key-bindings.zsh \
      zsh-users/zsh-autosuggestions \
      zdharma/fast-syntax-highlighting \
      zsh-users/zsh-history-substring-search \
      molovo/tipz \
      changyuheng/zsh-interactive-cd \
      frmendes/geometry \
      jedahan/geometry-hydrate
    source ~/.zr/init.zsh # ~160ms

#### benchmarks

    cargo +nightly bench

You can also bench a minimal zsh loading, with

    time zsh -i -c "source \"$PWD\"/benches/zshrc-zr; zprof"

#### developing

    cargo +nightly build --features="clippy"

[init.zsh]: https://github.com/jedahan/dotfiles/blob/master/.zshrc

#### thanks

- [SX91](https://github.com/SX91) for linux fixes
- [alanpearce](https://github.com/alanpearce) for bug reports and nix package
