#!/bin/sh

# Creates a GitHub PR using the GitHub CLI

TITLE=$1

gh pr create --title "$TITLE" --body ""