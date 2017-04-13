#!/bin/bash
set -e

DST="/media/jeremy/JEREMY/certification/"
if [ ! -d "$DST" ]
then
  git clone "${PWD}" "${DST}"
fi

pushd "${DST}"
git pull
popd

sync
