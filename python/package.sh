#!/bin/bash

set -eu

rm -rf quizdown/quizdown
source venv/bin/activate
pip install -q -r requirements.txt
pip install -q -r dev-requirements.txt
maturin build --release
maturin develop --release
python -m quizdown --help
