# SPDX-License-Identifier: GPL-2.0

TARGET_MODULE := dev_scream
KVER ?= $(shell uname -r)
KDIR ?= /lib/modules/$(KVER)/build
EXTRA_DIR ?= /lib/modules/$(KVER)/updates

default: build

clean: unload uninstall

build:
	$(MAKE) -C $(KDIR) M=$$PWD modules

install: build
	$(MAKE) -C $(KDIR) M=$$PWD modules_install

load: install
	modprobe $(TARGET_MODULE)

unload:
	modprobe -r $(TARGET_MODULE)

uninstall:
	rm $(EXTRA_DIR)/$(TARGET_MODULE).ko