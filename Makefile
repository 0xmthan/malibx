VENV    := .venv
PYTHON  := $(VENV)/bin/python
MATURIN := $(VENV)/bin/maturin
MKDOCS  := $(VENV)/bin/mkdocs
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

$(MKDOCS): $(MATURIN) docs/requirements.txt
	uv pip install -q --python $(VENV) -r docs/requirements.txt
	touch $(MKDOCS)

docs: $(MKDOCS)
	$(MKDOCS) build --strict

docs-serve: $(MKDOCS)
	$(MKDOCS) serve

test:
	cargo test

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo test
	cargo audit

clean:
	rm -rf target dist site

fclean: clean
	rm -rf $(VENV)

re: fclean all

.PHONY: all dev run wheel docs docs-serve test lint clean fclean re
