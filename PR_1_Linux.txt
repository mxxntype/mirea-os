#!/usr/bin/env bash

SURNAME="Vartanyan"
NAME="Artem"
FNAME="Aleksandrovich"
GROUP="BSBO_01_22"

if [[ "$1" == "--clean" ]]; then
  rm -rf $SURNAME $FNAME $GROUP
  exit 0
fi

LS="ls -GgAhF"

pwd
mkdir -pv $FNAME $SURNAME $GROUP
$LS

cd $SURNAME
touch ${NAME}_{1,2,3}
$LS

WORDS="some\nsort\nof\nword\nthat\ni\nhad\nto\nput\ninto\nthis\nfile..."
for file in $(ls); do
  echo -e $WORDS > $file
done

cp ${NAME}_1 ../${FNAME}
mv ${NAME}_2 ../${GROUP}

cd ..
cat */*

cd $GROUP
mv ${NAME}_2 "${SURNAME}_${NAME}"
cat "${SURNAME}_${NAME}"
cd ..

cp -R ${FNAME} ${SURNAME}/
cat ${SURNAME}/*

cp -R ${GROUP} ${SURNAME}/${FNAME}/
$LS ${SURNAME}/${FNAME}

cd ${FNAME}
cp ${NAME}_1 ${NAME}_2
cp ${NAME}_1 ${NAME}_3
$LS
mv ${NAME}_1 Сентябрь
mv ${NAME}_2 Октябрь
mv ${NAME}_3 Ноябрь
cd ..

rm -rf $SURNAME/*
$LS
$0 --clean
$LS
