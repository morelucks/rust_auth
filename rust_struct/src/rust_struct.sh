#!/usr/bin/ sh

BINARY_FILE=""

echo "downloading binary file"

wget  "$BINARY_FILE"

echo "download completed."

chmod +x cpc_rust

sudo mv cpc_rust "/usr/local/bin/luckify"


