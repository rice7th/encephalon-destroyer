PREFIX?=/usr

install:
	cp -f target/release/encephalon-destroyer $(PREFIX)/bin/swefencephalon-destroyer
uninstall:
	rm -f $(PREFIX)/bin/encephalon-destroyer
