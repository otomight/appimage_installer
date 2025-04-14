#!/bin/bash

updateDesktopFile() {
	local folder
	local desktop_file
	local icon_file
	local exe_file

	folder=$1
	desktop_file=$2
	icon_file=$3
	exe_file=$(find "$folder" -maxdepth 1 -name \
		"$APP_IMAGE_EXE_NAME" -print -quit)
	sed -i "s|^Exec=.*|Exec=\"$exe_file\"|g" "$desktop_file"
	sed -i "s|^Icon=.*|Icon=$icon_file|g" "$desktop_file"
}

suggestFuse() {
	if ! command -v fusermount &> /dev/null; then
		print "$MSG_INSTALLATION_FUSE"
	fi
}

setupFiles() {
	local folder
	local desktop_file
	local icon_file

	folder=$1
	desktop_file=$(find "$folder" -maxdepth 1 -name \
		"*.$DESKTOP_EXT" -print -quit)
	icon_file=$(find "$folder" -maxdepth 1 -name "*.$ICON_EXT" -print -quit)
	updateDesktopFile "$folder" "$desktop_file" "$icon_file"
	if ! [ -d "$DESKTOP_DIR" ]; then
		mkdir -p "$DESKTOP_DIR"
	fi
	cp "$desktop_file" "$DESKTOP_DIR"
	print "$(MSG_FILE_MOVED "$(basename "$desktop_file")" "$DESKTOP_DIR")"
	print "$(MSG_APP_INSTALLED "$(basename "$folder")")"
	suggestFuse
}
