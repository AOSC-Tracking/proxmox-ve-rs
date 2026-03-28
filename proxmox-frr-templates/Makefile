include /usr/share/dpkg/default.mk

PACKAGE=proxmox-frr-templates

GITVERSION:=$(shell git rev-parse HEAD)

BUILDDIR ?= build/$(PACKAGE)-$(DEB_VERSION)
DSC=$(PACKAGE)_$(DEB_VERSION).dsc

PVE_DEB=$(PACKAGE)_$(DEB_VERSION_UPSTREAM_REVISION)_all.deb

DEBS=$(PVE_DEB)

all: deb
deb: $(DEBS)

$(BUILDDIR):
	rm -rf $@ $@.tmp
	mkdir -p $@.tmp
	cp -a -t $@.tmp debian templates
	echo "git clone git://git.proxmox.com/git/proxmox-ve-rs.git\\ngit checkout $(GITVERSION)" > $@.tmp/debian/SOURCE
	mv $@.tmp $@

$(PVE_DEB): $(BUILDDIR)
	cd $(BUILDDIR); dpkg-buildpackage -b -uc -us
	lintian build/$(DEBS)

dsc: $(DSC)
	$(MAKE) clean
	$(MAKE) $(DSC)
	lintian build/$(DSC)

$(DSC): $(BUILDDIR)
	cd $(BUILDDIR); dpkg-buildpackage -S -uc -us

sbuild: $(DSC)
	sbuild build/$(DSC)

.PHONY: upload
upload: UPLOAD_DIST ?= $(DEB_DISTRIBUTION)
upload: $(DEBS)
	tar cf - build/$(DEBS)|ssh repoman@repo.proxmox.com -- upload --product devel --dist $(UPLOAD_DIST)

.PHONY: distclean
distclean: clean

.PHONY: clean
clean:
	rm -rf build

