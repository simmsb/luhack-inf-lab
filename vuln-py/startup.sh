#!/bin/sh

set -e

adduser -g "" smooth -D -s /bin/sh
passwd root -d $ROOT_PW && ssh-keygen -f /home/smooth/root -N $ROOT_PW

su - smooth -c "echo $FLAG_0 > /home/smooth/flag.txt"
echo $FLAG_1 > /root/flag.txt

su - smooth -c server
