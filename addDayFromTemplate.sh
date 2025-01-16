#! /bin/zsh
# ./addDayFromTemplate.sh 2023 1

YEAR=$1
DAY=$(printf "%02d" "$2")

RUST_SRC_FOLDER="./src/year$YEAR/day$DAY"
FILE_PART1="$RUST_SRC_FOLDER/lib/part1.rs"
FILE_PART2="$RUST_SRC_FOLDER/lib/part2.rs"
FILE_PARSER="$RUST_SRC_FOLDER/lib/parser.rs"
FILE_MOD="$RUST_SRC_FOLDER/lib/mod.rs"
INPUTS_FOLDER_PATH="./input/$YEAR/$DAY/"

if [ -f "$FILE_PART1" ]; then
    echo "$FILE_PART1 already exists."
    exit 1
fi

if [ -f "$FILE_PART2" ]; then
    echo "$FILE_PART2 already exists."
    exit 1
fi

if [ -f "$FILE_PARSER" ]; then
    echo "$FILE_PARSER already exists."
    exit 1
fi

if [ -f "$FILE_MOD" ]; then
    echo "$FILE_MOD already exists."
    exit 1
fi

mkdir -p "$RUST_SRC_FOLDER/lib"
cp ./template/part.rs.template "$FILE_PART1"
cp ./template/part.rs.template "$FILE_PART2"
cp ./template/parser.rs.template "$FILE_PARSER"
cp ./template/mod.rs.template "$FILE_MOD"
echo "pub mod day$DAY { pub mod lib; }" >> "src/year$YEAR.rs"

gsed -i "s/<DAY>/$DAY/g" "$FILE_PART1"
gsed -i "s/<YEAR>/$YEAR/g" "$FILE_PART1"
gsed -i "s/<PART_NUMBER>/1/g" "$FILE_PART1"
gsed -i "s/<DAY>/$DAY/g" "$FILE_PART2"
gsed -i "s/<YEAR>/$YEAR/g" "$FILE_PART2"
gsed -i "s/<PART_NUMBER>/2/g" "$FILE_PART2"
gsed -i "s/<DAY>/$DAY/g" "$FILE_PARSER"
gsed -i "s/<YEAR>/$YEAR/g" "$FILE_PARSER"
gsed -i "s/<PART_NUMBER>/2/g" "$FILE_PARSER"
gsed -i "s/<DAY>/$DAY/g" "$FILE_MOD"
gsed -i "s/<YEAR>/$YEAR/g" "$FILE_MOD"
gsed -i "s/<PART_NUMBER>/2/g" "$FILE_MOD"

mkdir -p "$INPUTS_FOLDER_PATH"
touch "$INPUTS_FOLDER_PATH/inputs.txt"
touch "$INPUTS_FOLDER_PATH/test_inputs_part1.txt"
touch "$INPUTS_FOLDER_PATH/test_inputs_part2.txt"

curl "https://adventofcode.com/$YEAR/day/$2/input" --cookie "session=$(cat ./cookieSession.txt)" > "$INPUTS_FOLDER_PATH/inputs.txt"
curl -s "https://adventofcode.com/$YEAR/day/$2" --cookie "session=$(cat ./cookieSession.txt)"  | gsed -n '/<pre><code>/,/<\/code><\/pre>/p' | gsed -e 's/<pre><code>//g' -e 's/<\/code><\/pre>//g' > "$INPUTS_FOLDER_PATH/test_inputs_part1.txt"
cp "$INPUTS_FOLDER_PATH/test_inputs_part1.txt" "$INPUTS_FOLDER_PATH/test_inputs_part2.txt"
