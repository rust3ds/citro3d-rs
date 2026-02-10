#!/usr/bin/env bash

# Get the asset directory (containing this script)
ASSET_DIR=$(dirname $0)

echo "Kitten"
tex3ds -f auto-etc1 -z auto -o $ASSET_DIR/kitten.t3d $ASSET_DIR/kitten.png
echo "Skybox"
tex3ds -f auto-etc1 --skybox -z auto -o $ASSET_DIR/skybox.t3d $ASSET_DIR/skybox.png
