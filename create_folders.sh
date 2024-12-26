#!/bin/bash

for i in {1..25}
do
    mkdir -p "src/day_$i"
    touch "src/day_$i/part_1.rs"
    touch "src/day_$i/part_2.rs"
done