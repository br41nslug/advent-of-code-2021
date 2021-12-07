# bash solution for day 1 part 2
IFS=$'\n'
set -f
PREV=''
INP="$(cat /dev/stdin)"
list=($(printf "%s" "$INP"))
MXI=$((${#list[@]}-2))
COUNTER=0
for ((i = 0 ; i < $MXI ; i++)); do
  ITEM=$((${list[$i]}+${list[$i+1]}+${list[$i+2]}))
  if [ ! -z $PREV ] && [[ $PREV -lt $ITEM ]]; then
    ((++COUNTER))
  fi
  PREV=$ITEM
done
echo $COUNTER