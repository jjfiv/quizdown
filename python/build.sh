#!/bin/bash

set -eu

rm -rf quizdown/quizdown
source venv/bin/activate
pip install -q -r dev-requirements.txt
maturin build 
maturin develop
python -m unittest discover -s tests
python -m quizdown --help
