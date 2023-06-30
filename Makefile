maelstrom.tar.bz2:
	curl -L "https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2" -o  maelstrom.tar.bz2

maelstrom:
	tar -xvf maelstrom.tar.bz2
	mv maelstrom/maelstrom .
	chmod +x maelstrom

.PHONY: setup
setup: maelstrom

.PHONY: challenge-1
challenge-1:
	cargo build --bin challenge-1
	./maelstrom/maelstrom test -w echo --bin target/debug/challenge-1 --node-count 1 --time-limit 10
