#!/usr/bin/env nu

def main [x: string] {
    mkdir $"../day-($x)"
    mkdir $"../day-($x)/rust"
    cd $"../day-($x)/rust"
    cargo init
    rm -rf src
    cp -r .../templates/rust src
}
