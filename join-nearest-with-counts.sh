#!/bin/sh -e

cat cities.txt | while read cityLat cityLong city; do
    exec > result/nearest/$city.resolved.txt
    cat result/nearest/$city.stations.txt | while read stationId lat long info; do
        count=NO
        if test -e work/$stationId.count; then
            count=$(cat work/$stationId.count)
        fi
        echo $stationId $count $lat $long $info
    done
done

# vim: set ts=4 sw=4 et:
