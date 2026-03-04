#!/bin/bash

## No prerequisites needed - uses Python's built-in JSON module

DENALI_SCEN_FILES=$(find . -name "*.scen.json")
DENALI_STEP_FILES=$(find . -name "*.step.json")
DENALI_STEPS_FILES=$(find . -name "*.steps.json")
DENALI_ALL_FILES="$DENALI_SCEN_FILES $DENALI_STEP_FILES $DENALI_STEPS_FILES"

for DENALI_FILE in $DENALI_ALL_FILES
do
    echo $DENALI_FILE
    python3 -m json.tool --indent 4 "$DENALI_FILE" > "$DENALI_FILE.tmp" && mv "$DENALI_FILE.tmp" "$DENALI_FILE" || exit 1
done
