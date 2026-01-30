mod ariadna

default:
    @just --list --unsorted

init:
    espup install --std --targets esp32s3

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
