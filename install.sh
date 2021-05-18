#!/bin/sh
# Based on the Deno Installer: https://github.com/denoland/deno_install/blob/master/install.sh

set -e

if ! command -v unzip >/dev/null; then
	echo "Error: unzip is required to install Scoob." 1>&2
	exit 1
fi

if [ "$OS" = "Windows_NT" ]; then
	target="x86_64-pc-windows-msvc"
else
	case $(uname -sm) in
	"Darwin x86_64") target="x86_64-apple-darwin" ;;
	"Darwin arm64") target="x86_64-apple-darwin" ;;
	# TODO: Once libsodium is updated, use the native arm64 binary for macs:
	# "Darwin arm64") target="aarch64-apple-darwin" ;;
	*)
		case $(uname -m) in
		"x86_64") target="x86_64-unknown-linux-gnu" ;;
		*) target="aarch64-unknown-linux-gnu" ;;
		esac
	esac
fi

echo ${target}

if [ $# -eq 0 ]; then
	scoob_uri="https://github.com/hostyhosting/scoob-rs/releases/latest/download/scoob-${target}.zip"
else
	scoob_uri="https://github.com/hostyhosting/scoob-rs/releases/download/${1}/scoob-${target}.zip"
fi

scoob_install="${SCOOB_INSTALL:-$HOME/.scoob}"
bin_dir="$scoob_install/bin"
exe="$bin_dir/scoob"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe.zip" "$scoob_uri"
unzip -d "$bin_dir" -o "$exe.zip"
chmod +x "$exe"
rm "$exe.zip"

echo "Scoob was installed successfully to $exe"
if command -v scoob >/dev/null; then
	echo "Run 'scoob --help' to get started"
else
	case $SHELL in
	/bin/zsh) shell_profile=".zshrc" ;;
	*) shell_profile=".bash_profile" ;;
	esac
	echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
	echo "  export SCOOB_INSTALL=\"$scoob_install\""
	echo "  export PATH=\"\$SCOOB_INSTALL/bin:\$PATH\""
	echo "Run '$exe --help' to get started"
fi
