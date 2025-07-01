seq 1 5 \
    | /usr/bin/wc -l \
    | /usr/bin/grep 5  > /dev/null
seq 1 5 \
    | /usr/bin/head -n 1 \
    | /usr/bin/grep 1  > /dev/null

seq 5 1 \
    | /usr/bin/head -n 1 \
    | /usr/bin/grep 5  > /dev/null

seq 2 2 \
    | /usr/bin/head -n 1 \
    | /usr/bin/grep 2  > /dev/null
seq 2 2 \
    | /usr/bin/wc -l \
    | /usr/bin/grep 1  > /dev/null
