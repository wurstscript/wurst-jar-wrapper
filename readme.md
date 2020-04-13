The wurst jar wrapper is an executable that calls into java and a jar file.
It's made to enable wurst jars to be executed from the command line on Windows platforms.

At present, this is used for two purposes:

- wurstscript.exe
- grill.exe

[![Build Status](https://travis-ci.org/wurstscript/wurst-jar-wrapper.svg?branch=master)](https://travis-ci.org/wurstscript/wurst-jar-wrapper)
[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/1985/badge)](https://bestpractices.coreinfrastructure.org/projects/1985)


# Test the wrapper

There are no tests, but `cargo run` will compile and run the wrapper.


# Installation

* `rustup update`
* `cargo install --force cargo-make`
* `cargo make wurstscript`
* `cp target/release/wurst-jar-wrapper.exe ~/.wurst/wurstscript.exe`
* `cargo make grill`
* `cp target/release/wurst-jar-wrapper.exe ~/.wurst/grill.exe`


# History

The wurst jar wrapper was originally a component of wurstpack (WurstScript's fork of JNGP).

Now it is used to wrap multiple jar files.


# Issues?

- [Raise an issue in the github issue tracker](https://github.com/wurstscript/wurst-jar-wrapper/issues)
- [Send an instant message on matrix.org:#wurst](https://riot.im/app/)
