.PHONY: all clean default install lock update check pc test docs run

default: check

install:
	pre-commit install
	uv sync
lock:
	uv lock
update:
	uv sync --upgrade

check: pc lint test
pc:
	pre-commit run -a
lint:
	uv run ruff check .
	uv run ruff format .
	uv run mypy .
	uv run pyright .
test:
	uv run pytest -m 'not online'

doc:
	uv run mkdocs serve

# make nuitka OUTPUT_FILE=images-upload-cli.exe
nuitka:
	uv run nuitka \
	  --assume-yes-for-downloads \
	  --onefile \
	  --output-dir=dist \
	  --output-file=$(OUTPUT_FILE) \
	  --script-name=src/images_upload_cli/__main__.py

bumped:
	git cliff --bumped-version

# make release TAG=$(git cliff --bumped-version)-alpha.0
release: check
	git cliff -o CHANGELOG.md --tag $(TAG)
	pre-commit run --files CHANGELOG.md || pre-commit run --files CHANGELOG.md
	git add CHANGELOG.md
	git commit -m "chore(release): prepare for $(TAG)"
	git push
	git tag -a $(TAG) -m "chore(release): $(TAG)"
	git push origin $(TAG)
