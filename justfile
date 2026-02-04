mod firmware
mod chips

default:
    @just --list --unsorted

init: install-esp32-toolchain

fmt:
    treefmt --ci

clean:
    just firmware::clean
    just chips::clean

check:
    just firmware::check
    just chips::check

build:
    just firmware::build
    just chips::build

test:
    just firmware::test
    just chips::test

install-esp32-toolchain:
    espup install --std --targets esp32s3
