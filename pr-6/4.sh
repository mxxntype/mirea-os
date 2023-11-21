#!/usr/bin/env bash

if [ $# -ne 1 ]; then
  echo "Использование: $0 <имя архива без расширения>"
  exit 1
fi

ARCHIVE_NAME="./$1_$(date +'%Y_%H_%M_%S').zip"
find ~/ -name "*.txt" -exec zip -q $ARCHIVE_NAME {} +
echo "Архив $ARCHIVE_NAME создан успешно."
