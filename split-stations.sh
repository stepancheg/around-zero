#!/bin/sh -e

rm -f work/*.csv

lastStationId=

while read line; do
    stationId=${line%%,*}
    if [ "$lastStationId" != "$stationId" ]; then
        echo $stationId >&2
        exec > work/$stationId.csv
        lastStationId=$stationId
    fi
    echo $line
done < 2012.csv.filtered.sorted-by-station

# vim: set ts=4 sw=4 et:
