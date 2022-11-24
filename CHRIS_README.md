


python3 -m venv .env
source .env/bin/activate

maturin develop -- -Awarnings

python3 -i model2.py
