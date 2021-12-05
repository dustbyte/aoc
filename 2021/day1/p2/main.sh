#!/bin/sh

if [ $# -lt 1 ]; then
  echo "usage: $0 file" >& 2
  exit 255
fi

filename="$1"
window=""
prev=""
cur=""
counter=0

for num in $(cat "$filename"); do
  window="$window $num"
  wsize=$(echo "$window" | wc -w)

  if [ "$wsize" -eq 4 ]; then
    window=$(echo $window | cut -d ' ' -f 2-)
  fi

  if [ "$wsize" -ge 3 ]; then
    cur=$(echo "$window" | xargs | sed 's/ /+/g' | bc)
    if [ "$prev" != "" ] && [ "$cur" -gt "$prev" ]; then
      counter=$(($counter + 1))
    fi
    prev="$cur"
  fi
done

echo $counter
