#!/usr/bin/env nu

def main [x: string, type: string = "rust"] {
    mkdir $"($env.PWD)/day-($x)"
    if $type == "rust" {
        mkdir $"($env.PWD)/day-($x)/rust"
        cd $"($env.PWD)/day-($x)"
        aoc download -d $x
        cd $"($env.PWD)/rust"
        cargo init
        rm -rf src
        cp -r .../templates/rust src
        cp ../input src/data.txt
        cd ...
    }
}
