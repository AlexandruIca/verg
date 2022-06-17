#!/usr/bin/env sh

set -e

xelatex main.tex
biber main
xelatex main.tex
xelatex main.tex
