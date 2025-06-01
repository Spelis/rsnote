APP := rsnote
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release

OSXCROSS_ROOT := /opt/osxcross
MACOS_SDK := $(OSXCROSS_ROOT)/tarballs/MacOSX11.3.sdk.tar.xz

export PATH := $(OSXCROSS_ROOT)/target/bin:$(PATH)

TARGETS := \
	x86_64-unknown-linux-gnu \
	x86_64-apple-darwin \
	x86_64-pc-windows-gnu

BIN_NAMES := \
	$(RELEASE_DIR)/$(APP)-linux \
	$(RELEASE_DIR)/$(APP)-macos \
	$(RELEASE_DIR)/$(APP)-windows.exe

all: $(BIN_NAMES)

$(RELEASE_DIR)/$(APP)-linux: x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/$(APP) $@

$(RELEASE_DIR)/$(APP)-macos: x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/$(APP) $@

$(RELEASE_DIR)/$(APP)-windows.exe: x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/$(APP).exe $@

x86_64-unknown-linux-gnu:
	cargo build --release --target $@

x86_64-apple-darwin: check-osxcross
	@echo "Building for macOS x86_64..."
	$(MAKE) crossbuild TARGET=$@

x86_64-pc-windows-gnu:
	@echo "Building for Windows..."
	CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ cargo build --release --target $@

crossbuild:
	rustup target add $(TARGET)
	CC=o64-clang CXX=o64-clang++ cargo build --release --target=$(TARGET)

check-osxcross:
	@if [ ! -d "$(OSXCROSS_ROOT)" ]; then \
		echo "Missing osxcross. Install it at $(OSXCROSS_ROOT)"; \
		exit 1; \
	fi

clean:
	cargo clean

list:
	@echo "Available targets:"
	@echo $(TARGETS) | tr ' ' '\n'
