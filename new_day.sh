#!/bin/bash
read -p 'Enter name of day: ' day

if [ -f "src/days/$day.rs" ];
then
    echo "Day $day already created"
    exit 0
fi

cp "src/days/day.rs.template" "src/days/$day.rs"
echo "pub mod $day;" >> "src/days/mod.rs"