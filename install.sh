#!/bin/bash

println() {
    printf '%s\n' "$*"
}

println "Kirino Media Server Installation Utility"

download() {
    if ! mkdir .kirino_source; then
        println "Trying again..."
        rm -r .kirino_source/
        mkdir .kirino_source
    fi

    if ! git clone https://github.com/kirino-org/kirino .kirino_source/; then
        println "Failed to clone the Git repo."
        exit 1
    else
        println "Git repo cloned! Proceeding to build..."
        cd .kirino_source/
    fi
}

build_err() {
    printf 'Failed trying to build "%s".\n' "$*"
    exit 1
}

go_build() {
    if ! mkdir bin; then
        println "Trying again..."
        rm -r bin/
        mkdir bin
    fi

    go build -v -a -buildvcs -o ./bin/server ./cli || build_err "cli"
}

build() {
    if ! command -v go >/dev/null; then
        printf "\nGo isn't installed. Please run one of the following:\n"
        printf " Gentoo: emerge --ask dev-lang/go\n Arch:   pacman -Sy go\n Debian: apt install go\n Fedora: dnf install go\n"
        exit 1
    else
        printf "\n"
        println "Go is installed!"
        go version
        printf "\n"

        println "Building..."
        go_build
    fi

    printf "Done!"
}

download
build
