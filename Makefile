PREFIX?=/usr

install:
	cp -f target/release/ed $(PREFIX)/bin/swef
uninstall:
	rm -f $(PREFIX)/bin/swef
