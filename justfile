
@default:
	just --list

run DAY:
	cargo run {{DAY}} 

test DAY:
	cargo test day{{DAY}}

watch DAY:
	cargo watch -c -x "test {{DAY}}"
