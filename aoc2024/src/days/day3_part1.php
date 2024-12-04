<?php
$loadFrom = "../../inputs/3/input.txt";

if(!file_exists($loadFrom)) {
  echo "Missing $loadFrom\n";
  exit(1);
}

$input = file_get_contents($loadFrom);

$lookFor = "mul(";
$lookFor = str_split($lookFor);
$arr = str_split($input);

$parsed = "";
$valid = true;
// Find all where there is the word mul

$validParsed = 0;
$nums = [];
for($i = 0; $i < count($arr); $i++) {
  if(@$lookFor[$validParsed] == $arr[$i]) {
    $validParsed++;
    echo "Match for $validParsed - " . $arr[$i] . "\n";
  } // Whole part has matched this far
  else if($validParsed == 4) {
    $validParsed = 0;
    // Get part until NEXT )
    $remainder = substr($input, $i);
    $next = strpos($remainder, ")");
    if($next == false || $next > 7) {
    } else {
      $nums[] = substr($input, $i, $next);
    }
  }
  else {
    echo "No match for " . $arr[$i] . "\n";
    $validParsed = 0;
  }
}

$muls = [];
foreach($nums as $mul) {
  $x = explode(",", $mul);
  $muls[] = $x;
}

$total = 0;
foreach($muls as $mul) {
  $total += (int)$mul[0] * (int)$mul[1];
}

var_dump($total);




