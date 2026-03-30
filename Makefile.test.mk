.PHONY: test run

HOSTINGS := beeimg catbox fastpic freeimage gyazo imageban imagebin imgbb imgchest imgur lensdump pixeldrain pixhost ptpimg sxcu thumbsnap tixte uplio uploadcare vgy zpic
TEST_IMG := tests/fixtures/image.png

run:
	cargo run --quiet -- --help

test:
	@cargo build --quiet
	@for h in $(HOSTINGS); do \
		printf "%-12s " "$$h:"; \
		./target/debug/imgup --hosting $$h --no-clipboard $(TEST_IMG); \
	done
