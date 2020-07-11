#!/usr/bin/env sh

set -o errexit
set -o nounset

cd node
git fetch
MOST_RECENT_VERSION=$(git tag | sort -V | tail -n 1 )
CURRENT_VERSION=$(git describe --tags HEAD)

if [ "$MOST_RECENT_VERSION" = "$CURRENT_VERSION" ]; then
  echo "$CURRENT_VERSION is up to date, nothing to do"
  exit 0
fi

CHANGED_FILES=$(git diff --name-only "$CURRENT_VERSION".."$MOST_RECENT_VERSION" src/node_api.h src/js_native_api.h src/js_native_api_types.h src/node_api_types.h)

if [ -z "$CHANGED_FILES" ]; then
  echo "Relevant files did not change between $CURRENT_VERSION..$MOST_RECENT_VERSION"
  exit 0
else
  IFS="
  "

  CHANGELOG=$(mktemp)
  echo "# Node ${MOST_RECENT_VERSION}\n" >> $CHANGELOG
  echo "affected files:" >> $CHANGELOG
  for i in $CHANGED_FILES; do
    echo "* [$i](https://github.com/nodejs/node/blob/$MOST_RECENT_VERSION/$i)" >> $CHANGELOG
  done

  echo "\nupstream changelog:" >> $CHANGELOG
  for i in "$(git log --format='[%h](https://github.com/nodejs/node/commit/%H) %s' "$CURRENT_VERSION".."$MOST_RECENT_VERSION" src/node_api.h src/js_native_api.h src/js_native_api_types.h src/node_api_types.h)"; do
    echo "* $i" >> $CHANGELOG
  done

  echo "" >> $CHANGELOG
fi

git checkout "$MOST_RECENT_VERSION"

if ! grep "NAPI_VERSION  6" src/node_version.h; then
  echo "Detected new NAPI_VERSION. Refusing to auto-update. Please add feature-flag etc."
  exit 1
fi

# back to src-root
cd ..
cat CHANGELOG.md >> $CHANGELOG
cat $CHANGELOG > CHANGELOG.md
git commit -m ":arrow_up: update node $CURRENT_VERSION -> $MOST_RECENT_VERSION" node CHANGELOG.md
