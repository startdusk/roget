.PHONY: codeline
codeline:
	@tokei .

.PHONY: test 
test: fmt
	@cargo nextest run

.PHONY: fmt
fmt:
	@cargo clippy

.PHONY: roget/r
roget/r: fmt
	@cargo r --release --  --implementation naive

.PHONY: roget/pid
roget/pid:
	@pgrep -af roget

.PHONY: roget/hyperfine3
roget/hyperfine3:
	@cargo build --release
	@hyperfine \
		-n naive './target/release/roget --implementation naive --max 1' \
		-n allocs './target/release/roget --implementation allocs --max 1' \
		-n vecremain './target/release/roget --implementation vecremain --max 1' \
		