#!/bin/sh -e

egrep -w 'TMIN|TMAX' ./2012.csv > 2012.csv.filtered

# vim: set ts=4 sw=4 et:
