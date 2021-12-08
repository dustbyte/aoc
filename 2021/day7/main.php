#!/usr/bin/env php
<?php

if (count($argv) < 2) {
    fwrite(STDERR, "usage: $argv[0] file\n");
    exit(-1);
}

$handle = fopen($argv[1], "r");
$line = fgets($handle);
fclose($handle);

$string_list = explode(",", $line);

$list = [];
foreach ($string_list as $number) {
    array_push($list, (int)$number);
}

sort($list);

$median = $list[count($list)/2];
$avg  = floor(array_sum($list) / count($list));

$sum = 0;
$sumsum = 0;
foreach($list as $number) {
    $sum += abs($number - $median);
    $fuel = (abs($number - $avg));
    $sumsum += ($fuel * ($fuel + 1)) / 2;
}

echo $sum . "\n";
echo $sumsum . "\n";
?>
