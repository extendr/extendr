# just: https://github.com/casey/just

default:
    @just --list

build-paper: docker run --env JOURNAL=joss  --volume $pwd/:/data openjournals/inara

# install via `cargo install watchexec-cli --force`
watch-paper: watchexec -w paper.md -w paper.bib #STILL MISSING build command

clean-paper: docker rm openjournals/inara
