PREFIX?=/usr

.install:
	cp -f target/release/ed $(PREFIX)/bin/swef
.uninstall:
	rm -f $(PREFIX)/bin/swef
.remove: # adding a remove alias so that people can easly use what they prefer
	rm -f $(PREFIX)/bin/swef