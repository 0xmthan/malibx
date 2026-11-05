VENV    := .venv
PYTHON  := $(VENV)/bin/python
MATURIN := $(VENV)/bin/maturin
EXAMPLE ?= hello

all: dev

$(MATURIN):
	uv venv -q --python 3.11 $(VENV)
	uv pip install -q --python $(VENV) maturin

dev: $(MATURIN)
	VIRTUAL_ENV=$(VENV) $(MATURIN) develop --uv

run: dev
	$(PYTHON) examples/$(EXAMPLE).py

wheel: $(MATURIN)
	$(MATURIN) build --release --out dist

test:
	cargo test

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo test
	cargo audit

clean:
	rm -rf target dist

fclean: clean
	rm -rf $(VENV)

re: fclean all

.PHONY: all dev run wheel test lint clean fclean re
