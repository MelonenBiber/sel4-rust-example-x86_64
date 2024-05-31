# Get directory of clean.sh (project root)
ROOT="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

rm -rf $ROOT/seL4/build
rm -rf $ROOT/seL4/install
rm -rf $ROOT/target
