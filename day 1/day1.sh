# bash solution for day 1
COUNTER=0
PREV=''
for ITEM in $(cat /dev/stdin); do
  if [ ! -z $PREV ] && [[ $PREV -lt $ITEM ]]; then
    ((++COUNTER))
  fi
  PREV=$ITEM
done
echo $COUNTER