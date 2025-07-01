echo "junk" > cat_junk.txt
cat cat_junk.txt | /usr/bin/grep junk > /dev/null
