#!/usr/bin/env bash
# test_cycle.sh — chunked encode → chunked restore → diff

set -euo pipefail

# Project paths
DATA_DIR="./data"
OUTPUT_DIR="./output"

INPUT_FILE="$DATA_DIR/enwik8"

DICT_DIR="$OUTPUT_DIR/dicts"            # will become a folder of chunk_###.dict
SDICT_DIR="$OUTPUT_DIR/sdicts"          # folder of chunk_###.sdict
ENCODED_DIR="$OUTPUT_DIR/encodings"     # folder of chunk_###.enc
RECON_FILE="$OUTPUT_DIR/reconstructed.txt"

# prepare output
rm -rf "$OUTPUT_DIR"
mkdir -p "$DICT_DIR" "$SDICT_DIR" "$ENCODED_DIR"

echo "🔍 Checking for input file at $INPUT_FILE"
if [[ ! -f "$INPUT_FILE" ]]; then
  echo "❌ ERROR: Input file not found at $INPUT_FILE"
  exit 1
fi

echo "🚀 Chunked encode → writing into:"
echo "   dicts   : $DICT_DIR/"
echo "   sdicts  : $SDICT_DIR/"
echo "   encs    : $ENCODED_DIR/"
./redumb encode \
  "$INPUT_FILE" \
  "$DICT_DIR" \
  "$SDICT_DIR" \
  "$ENCODED_DIR"

echo "🔄 Chunked restore → produces $RECON_FILE"
./redumb restore \
  "$DICT_DIR" \
  "$ENCODED_DIR" \
  "$RECON_FILE"

echo "🧮 Comparing original vs reconstructed..."
if diff -u "$INPUT_FILE" "$RECON_FILE"; then
  echo "✅ SUCCESS: reconstructed matches original"
else
  echo "❌ FAILURE: reconstructed differs from original"
  exit 1
fi

