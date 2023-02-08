#!/bin/bash

# Mode 2: Rank Modules
# echo $1 will return a file path only if 
# file path is github 
GITPATH = 'github'
if [["$1" == *"$GITPATH"*]]; then echo $1
else echo "Based on our requirements we will not proces this file because it has a npm file path"
