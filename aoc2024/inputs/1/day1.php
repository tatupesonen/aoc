<?php

if(!file_exists("./input.txt")) {
  echo "Missing input.txt\n";
  exit(1);
}

$input = file_get_contents("./input.txt");

$left = [];
$right = [];
$split_on = "   ";
$lines = explode("\n", $input);

foreach($lines as $line) {
  $split = explode($split_on, $line);
  $left[] = $split[0];
  $right[] = $split[1];
}

sort($left);
sort($right);

$distances = [];
for($i = 0; $i < count($left); $i++) {
  $distances[] = abs($left[$i] - $right[$i]);
}

var_dump(array_sum($distances));

?>