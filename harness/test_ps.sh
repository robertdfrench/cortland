ps \
    | /usr/bin/awk '$2 == "ps"' \
    | /usr/bin/wc -l \
    | /usr/bin/grep 1 > /dev/null
