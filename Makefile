VENV    := .venv
PYTHON  := $(VENV)/bin/python
MATURIN := $(VENV)/bin/maturin

all: dev

$(MATURIN):
	uv venv -q --python 3.11 $(VENV)
	uv pip install -q --python $(VENV) maturin

dev: $(MATURIN)
	VIRTUAL_ENV=$(VENV) $(MATURIN) develop --uv

run: dev
	$(PYTHON) examples/hello.py

wheel: $(MATURIN)
	$(MATURIN) build --release --out dist

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo audit

clean:
	rm -rf target dist

fclean: clean
	rm -rf $(VENV)

re: fclean all

.PHONY: all dev run wheel lint clean fclean re
