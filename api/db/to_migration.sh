#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <sql_file>"
    exit 1
fi

sql_file=$1
timestamp=$(date +"%Y%m%d%H%M%S")
cp "${sql_file}" ../migrations/"${timestamp}"_"${sql_file}"

echo "Files have been renamed and moved to the migrations directory."
