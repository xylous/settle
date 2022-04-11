#!/usr/bin/bash

# Create a man (GROFF) document from the human-unintelligible markdown manual

pandoc --standalone settle.1.md --to man --output=settle.1
