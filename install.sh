#!/bin/bash

println() {
    printf '%s\n' "$*"
}

println "Kirino Media Server Installation Utility"

download() {
    printf "Would you like to download Kirino's source? ('y' to download source, 'n' to quit now): "
    read -n 1 -r should_dl_source

    printf "\n\n"

    case $should_dl_source in
        'y')
            printf "Downloading source...\n"

            if ! mkdir ./.kirino_source; then
                println "Couldn't create '.kirino_source' directory to clone into."

                printf "Would you like to "

                exit 1
            fi

            if ! git clone https://github.com/kirino-org/kirino ./.kirino_source; then
                println "Couldn't clone 'https://github.com/kirino-org/kirino' into '.kirino_source'."
                exit 1
            else
                println "Success! Proceeding to build..."
            fi
            ;;
        #b)
        #    printf "Downloading binary..."
        #    ;;
        n)
            exit 1
            ;;
    esac
}
