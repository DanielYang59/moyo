set shell := ["zsh", "-uc"]
set positional-arguments

default:
    just --list

build-python:
    maturin develop --release --manifest-path moyopy/Cargo.toml

install-python:
    maturin develop --release --manifest-path moyopy/Cargo.toml
    pip install nv
    uv pip install -e "moyopy[dev]"
    pre-commit install

test-python:
    pytest -v moyopy/python/tests

pre-commit:
    pre-commit run --all-files

clean:
    rm moyopy/python/moyo/*.so
