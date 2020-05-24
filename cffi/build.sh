#!/bin/bash

set -eu

source venv/bin/activate
pip install -q -r dev-requirements.txt
maturin build -b cffi
maturin develop -b cffi && python -m quizdown_cffi
