#!/bin/sh

if [ $# -lt 1 ]; then
  echo "usage: $0 file" >& 2
  exit 255
fi

filename="$1"
prev=""
counter=0

for cur in $(cat "$filename"); do
  if [ "$prev" != "" ]; then
    if [ "$(($cur - $prev))" -gt 0 ]; then
      counter=$(($counter + 1))
    fi
  fi
  prev="$cur"
done

echo $counter
