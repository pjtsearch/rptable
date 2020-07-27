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
target/release/periodic_table : src
	cargo build --release

# Install onto the system
install : target/release/periodic_table
	# Create the bindir, if need be
	mkdir -p $(bindir)
	# Install binary
	$(INSTALL_PROGRAM) target/release/periodic_table $(bindir)/com.pjtsearch.periodic_table
	# Create the sharedir and subfolders, if need be
	mkdir -p $(sharedir)/icons/hicolor/scalable/apps/
	mkdir -p $(sharedir)/icons/hicolor/64x64/apps/
	mkdir -p $(sharedir)/icons/hicolor/128x128/apps/
	mkdir -p $(sharedir)/applications/
	mkdir -p $(sharedir)/metainfo/
	# Install icons
	$(INSTALL_DATA) data/com.pjtsearch.periodic_table.svg $(sharedir)/icons/hicolor/scalable/apps/com.pjtsearch.periodic_table.svg
	$(INSTALL_DATA) data/com.pjtsearch.periodic_table.64.png $(sharedir)/icons/hicolor/64x64/apps/com.pjtsearch.periodic_table.png
	$(INSTALL_DATA) data/com.pjtsearch.periodic_table.128.png $(sharedir)/icons/hicolor/128x128/apps/com.pjtsearch.periodic_table.png
	# Force icon cache refresh
	touch $(sharedir)/icons/hicolor
	# Install application meta-data
	$(INSTALL_DATA) data/com.pjtsearch.periodic_table.appdata.xml $(sharedir)/metainfo/com.pjtsearch.periodic_table.appdata.xml
	# Install desktop file
	$(INSTALL_DATA) data/com.pjtsearch.periodic_table.desktop $(sharedir)/applications/com.pjtsearch.periodic_table.desktop

# Remove an existing install from the system
uninstall :
	# Remove the desktop file
	rm -f $(sharedir)/applications/com.pjtsearch.periodic_table.desktop
	# Remove the application metadata
	rm -f $(sharedir)/metainfo/com.pjtsearch.periodic_table.appdata.xml
	# Remove the icon
	rm -f $(sharedir)/icons/hicolor/scalable/apps/com.pjtsearch.periodic_table.svg
	rm -f $(sharedir)/icons/hicolor/64x64/apps/com.pjtsearch.periodic_table.png
	rm -f $(sharedir)/icons/hicolor/128x128/apps/com.pjtsearch.periodic_table.png
	# Remove the binary
	rm -f $(bindir)/bin/com.pjtsearch.periodic_table

# Build a Flatpak package
flatpak-development: target/release/periodic_table
	mkdir -p flatpak-development
	flatpak-builder flatpak-development data/com.pjtsearch.periodic_table-development.json

flatpak-release : target/release/periodic_table
	mkdir -p flatpak
	flatpak-builder flatpak data/com.pjtsearch.periodic_table.json

# Remove all files
clean-all : clean
	cargo clean

deb : target/release/periodic_table
	mkdir -p deb
	mkdir -p deb/DEBIAN
	cp data/com.pjtsearch.periodic_table.control deb/DEBIAN/control
	make DESTDIR=deb install
	dpkg-deb --build deb deb/com.pjtsearch.periodic_table.deb

# Remove supplemental build files
clean :
	rm -rf flatpak/ flatpak-development/
	rm -rf deb/ 

