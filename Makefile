PREFIX?=/usr

install:
	cp -f target/release/encephalon-destroyer $(PREFIX)/bin/encephalon-destroyer
uninstall:
	rm -f $(PREFIX)/bin/encephalon-destroyer
