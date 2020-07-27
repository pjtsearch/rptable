# Install to /usr unless otherwise specified, such as `make PREFIX=/app`
PREFIX=/usr

# What to run to install various files
INSTALL=install
# Run to install the actual binary
INSTALL_PROGRAM=$(INSTALL)
# Run to install application data, with differing permissions
INSTALL_DATA=$(INSTALL) -m 644

# Directories into which to install the various files
bindir=$(DESTDIR)$(PREFIX)/bin
sharedir=$(DESTDIR)$(PREFIX)/share

# These targets have no associated build files.
.PHONY : clean clean-all install uninstall

# Build the application
target/release/rptable : src
	cargo build --release

# Install onto the system
install : target/release/rptable
	# Create the bindir, if need be
	mkdir -p $(bindir)
	# Install binary
	$(INSTALL_PROGRAM) target/release/rptable $(bindir)/com.pjtsearch.rptable
	# Create the sharedir and subfolders, if need be
	mkdir -p $(sharedir)/icons/hicolor/scalable/apps/
	mkdir -p $(sharedir)/icons/hicolor/64x64/apps/
	mkdir -p $(sharedir)/icons/hicolor/128x128/apps/
	mkdir -p $(sharedir)/applications/
	mkdir -p $(sharedir)/metainfo/
	# Install icons
	$(INSTALL_DATA) data/com.pjtsearch.rptable.svg $(sharedir)/icons/hicolor/scalable/apps/com.pjtsearch.rptable.svg
	$(INSTALL_DATA) data/com.pjtsearch.rptable.64.png $(sharedir)/icons/hicolor/64x64/apps/com.pjtsearch.rptable.png
	$(INSTALL_DATA) data/com.pjtsearch.rptable.128.png $(sharedir)/icons/hicolor/128x128/apps/com.pjtsearch.rptable.png
	# Force icon cache refresh
	touch $(sharedir)/icons/hicolor
	# Install application meta-data
	$(INSTALL_DATA) data/com.pjtsearch.rptable.appdata.xml $(sharedir)/metainfo/com.pjtsearch.rptable.appdata.xml
	# Install desktop file
	$(INSTALL_DATA) data/com.pjtsearch.rptable.desktop $(sharedir)/applications/com.pjtsearch.rptable.desktop

# Remove an existing install from the system
uninstall :
	# Remove the desktop file
	rm -f $(sharedir)/applications/com.pjtsearch.rptable.desktop
	# Remove the application metadata
	rm -f $(sharedir)/metainfo/com.pjtsearch.rptable.appdata.xml
	# Remove the icon
	rm -f $(sharedir)/icons/hicolor/scalable/apps/com.pjtsearch.rptable.svg
	rm -f $(sharedir)/icons/hicolor/64x64/apps/com.pjtsearch.rptable.png
	rm -f $(sharedir)/icons/hicolor/128x128/apps/com.pjtsearch.rptable.png
	# Remove the binary
	rm -f $(bindir)/bin/com.pjtsearch.rptable

# Build a Flatpak package
flatpak-development: target/release/rptable
	mkdir -p flatpak-development
	flatpak-builder flatpak-development data/com.pjtsearch.rptable-development.json

flatpak-release : target/release/rptable
	mkdir -p flatpak
	flatpak-builder flatpak data/com.pjtsearch.rptable.json

# Remove all files
clean-all : clean
	cargo clean

deb : target/release/rptable
	mkdir -p deb
	mkdir -p deb/DEBIAN
	cp data/com.pjtsearch.rptable.control deb/DEBIAN/control
	make DESTDIR=deb install
	dpkg-deb --build deb deb/com.pjtsearch.rptable.deb

# Remove supplemental build files
clean :
	rm -rf flatpak/ flatpak-development/
	rm -rf deb/ 

