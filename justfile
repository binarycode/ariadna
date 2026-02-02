mod ariadna

default:
    @just --list --unsorted

init: install-esp32-toolchain

fmt:
    treefmt --ci

clean:
    just ariadna::clean

check:
    just ariadna::check

build:
    just ariadna::build

test:
    just ariadna::test

install-esp32-toolchain:
    espup install --std --targets esp32s3
