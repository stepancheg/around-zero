#!/bin/sh -e

gen() {
    lat="$1"
    long="$2"
    city="$3"
    echo "generating nearest stations for $city"
    ./closest_stations "$lat" "$long" | head -100 > result/nearest/$city.stations.txt || true
}

cat cities.txt | while read lat long city; do
    if test -z "$1" || test "$city" == "$1"; then
        gen $lat $long $city
    fi
done

# vim: set ts=4 sw=4 et:
