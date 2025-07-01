/bin/rm -f junkA.txt junkB.txt

echo "junk" > junkA.txt
ln junkA.txt junkB.txt
/bin/cat junkB.txt | /usr/bin/grep junk > /dev/null

echo "knuj" > junkA.txt
/bin/cat junkB.txt | /usr/bin/grep knuj > /dev/null
