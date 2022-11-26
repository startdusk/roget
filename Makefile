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

.PHONY: roget/hyperfine4
roget/hyperfine4:
	@cargo build --release
	@hyperfine -m 5 \
		-n naive './target/release/roget --implementation naive --max 7' \
		-n allocs './target/release/roget --implementation allocs --max 7' \
		-n vecremain './target/release/roget --implementation vecremain --max 7' \
		-n once_init './target/release/roget --implementation once-init --max 7'
		