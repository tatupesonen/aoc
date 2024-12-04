<?php
$loadFrom = "../../../inputs/3/input.txt";

if(!file_exists($loadFrom)) {
  echo "Missing $loadFrom\n";
  exit(1);
}

$input = file_get_contents($loadFrom);

$lookForMul = "mul(";
$lookForMul = str_split($lookForMul);
$lookForDont = "don't()";
$lookForDont = str_split($lookForDont);
$lookForDo = "do()";
$lookForDo = str_split($lookForDo);
$arr = str_split($input);

$parsed = "";
$valid = true;

$validParsedMul = 0;
$validParsedDont = 0;
$validParsedDo = 0;
$nums = [];
$parsing = true;
for($i = 0; $i < count($arr); $i++) {
  if(@$lookForDo[$validParsedDo] == $arr[$i]) {
    $validParsedDo++;
    echo "Match for vDo $validParsedDo - " . $arr[$i] . "\n";
  } // Whole part has matched this far
  else if($validParsedDo == 4) {
    $parsing = true;
    echo "Start parsing\n";
    $validParsedDo = 0;
  } else {
    $validParsedDo = 0;
  }
  if(@$lookForDont[$validParsedDont] == $arr[$i]) {
    $validParsedDont++;
    echo "Match for vDont $validParsedDont - " . $arr[$i] . "\n";
  } // Whole part has matched this far
  else if($validParsedDont == 7) {
    $parsing = false;
    echo "Stop parsing\n";
    $validParsedDont = 0;
  }
  else {
    $validParsedDont = 0;
  }
  if($parsing) {
  if(@$lookForMul[$validParsedMul] == $arr[$i]) {
    $validParsedMul++;
    echo "Match for $validParsedMul - " . $arr[$i] . "\n";
  } // Whole part has matched this far
  else if($validParsedMul == 4) {
    $validParsedMul = 0;
    $remainder = substr($input, $i);
    $next = strpos($remainder, ")");
    if($next == false || $next > 7) {
    } else {
      $nums[] = substr($input, $i, $next);
    }
  }
  else {
    echo "No match for mul " . $arr[$i] . "\n";
    $validParsedMul = 0;
  }
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




