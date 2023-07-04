maelstrom.tar.bz2:
	curl -L "https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2" -o  maelstrom.tar.bz2

maelstrom/maelstrom: maelstrom.tar.bz2
	tar -xvmf maelstrom.tar.bz2

.PHONY: challenge-1
challenge-1: maelstrom/maelstrom
	cargo build --bin challenge-1
	./maelstrom/maelstrom test -w echo --log-stderr --bin target/debug/challenge-1 --node-count 1 --time-limit 10
