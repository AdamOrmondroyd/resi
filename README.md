# RESI
### Dark Energy Rusty Instrument
Rust port of BAO likelihoods for DESI.

## to pip install
```bash
pip install maturin
# or
uv tool install maturin

# pip install to currently active venv
maturin develop --release
```

## c++ interface
At this point all of the functionality is in compiled languages, so no need for python.

```bash
# checkout cpp branch and build
git checkout cpp
cargo build --release

# copy the header and library to polychord
cp target/cxxbridge/resi/src/lib.rs.h /path/to/polychord/likelihoods/CC/
cp target/release/libderi.a /path/to/polychord/likelihoods/CC/

# build and run polychord
cd /path/to/polychord
make clean; make polychord_CC
./bin/polychord_CC
```

My `rust` branch of `polychord` already has the requisite changes to use this specific library.
