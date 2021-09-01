set -o pipefail
cmd='cargo run'
#cmd='cargo run --release'
($cmd) 2>&1 | tee out
res=$?
echo res $res
if [ $res -ne 0 ] ; then
  less -S out
fi
