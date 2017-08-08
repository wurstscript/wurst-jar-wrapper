The wurst jar wrapper is a component of wurstpack (Wurstscript's fork of JNGP).

[![Build Status](https://travis-ci.org/wurstscript/wurst-jar-wrapper.svg?branch=master)](https://travis-ci.org/wurstscript/wurst-jar-wrapper)

# Test the wrapper

`cargo run -- --about`

# Installation

* `rustup default nightly`
* `rustup update`
* `cargo build --release`
* `cp target/release/wurst-jar-wrapper.exe ~/workspace/WurstScript/Wurstpack/wurstscript/wurstscript.exe`

# Configure

* `cp wrapper_config.toml.template ~/workspace/WurstScript/Wurstpack/wurstscript/wrapper_config.toml`
* Edit `wrapper_config.toml` to match your preferences.

# Manual usage

`wurstscript.exe ../Wurstpack/wurstscript/common.j ../Wurstpack/wurstscript/Blizzard.j ../../EBR/wurst -lib ../../StdLib2/ -lib ../../Frentity/`
