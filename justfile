# system requirement: 
# See https://just.systems/man/en/chapter_5.html or 
# ```
# scoop install just
# or
# cargo install just
# ```
# just: https://github.com/casey/just

set windows-shell := ["cmd.exe", "/c"]

default:
    @just --list

# Install dependencies necessary for building the paper
install-paper:
    cargo install watchexec-cli

# Build the paper using docker
build-paper: 
    docker run --env JOURNAL=joss  --volume %cd%/:/data openjournals/inara

# Continuously build paper if paper.[md|bib] is changed
watch-paper: 
    @watchexec -w paper.md -w paper.bib "just build-paper"

# Remove the openjournals/inara image from docker
[confirm("This will delete openjournals/inara image from your machine.")]
uninstall-paper: 
    docker rm openjournals/inara
