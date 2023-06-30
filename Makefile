maelstrom.tar.bz2:
	curl -L "https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2" -o  maelstrom.tar.bz2

maelstrom/maelstrom: maelstrom.tar.bz2
	tar -xvmf maelstrom.tar.bz2

.PHONY: challenge-1
challenge-1: maelstrom/maelstrom
	cargo build --bin challenge-1
	./maelstrom/maelstrom test -w echo --bin target/debug/challenge-1 --node-count 1 --time-limit 10

.PHONY: challenge-2
challenge-2: maelstrom/maelstrom
	cargo build --bin challenge-2
	./maelstrom/maelstrom test -w unique-ids --bin target/debug/challenge-2 --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
