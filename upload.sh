#!/bin/bash

set -euo pipefail

if [ $# != 1 ]
then
	echo "need 1 arguments: new version"
	echo -e "latest one is:\n$(git tag -n | sort -V | tail -n 1)"
	exit 1
fi

new_version=$1

set -x
cargo fmt
set +x

if [[ $(git status --porcelain 2> /dev/null | grep -v "??" | wc -l)  != "0" ]]
then
	echo "git workspace is dirty. Please commit your changes before tagging a version"
	exit 1
fi

set +e
cargo test
tests_ok=$?
set -e

if [ ${tests_ok} -eq 0 ] ; then
  echo "tests ok"
else
  echo "NOT UPLOADING! fix tests first. aborting"
  exit 1
fi  

if git show-ref --tags "$new_version" --quiet; then
  set +e
  current="$(git describe --exact-match --tags 2> /dev/null)"
  set -e
  if [ "$current" = "$new_version" ] ; then
    echo "uploading version ${new_version}"
  else
    read -p "tag ${new_version} exists but is not checked out! want to retag here? [y/N]" -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      git tag -a -f $new_version
    else
      echo "aborting"
      exit 1
    fi
  fi
else
  echo 
  read -p "tag ${new_version} doesn't exist. Want to tag now? [y/N] " -n 1 -r
  echo
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    git tag $new_version -m "$(git show --pretty="%s" |head -n 1)"
    git tag -f -a $new_version
  else
    echo "aborting"
    exit 1
  fi
fi

read -p "finally, push to github? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  set -x
  git push origin HEAD
  git push origin $new_version
  set +x
  echo "upload successful"
else
  echo "not uploaded"
fi

