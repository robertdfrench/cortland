echo "junk" > junk1.txt
cp junk1.txt junk2.txt
/bin/cat junk2.txt | /usr/bin/grep junk > /dev/null
