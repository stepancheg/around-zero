#!/bin/sh -e

for f in work/*.csv; do
    stationId=${f#work/}
    stationId=${stationId%.csv}
    echo $stationId
    ./around $stationId > work/$stationId.count
done

# vim: set ts=4 sw=4 et:
