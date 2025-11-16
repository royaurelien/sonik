#!/usr/bin/env bash
set -e

IMAGE="fake-device.img"
SIZE_MB=$((1024 * 2))  # 2GB
LABEL="FAKEDEVICE"

# Return the unique loop device associated to the image
get_loop() {
    LOOPS=($(losetup -j "$IMAGE" | cut -d: -f1))

    if [ ${#LOOPS[@]} -eq 0 ]; then
        echo ""
    elif [ ${#LOOPS[@]} -eq 1 ]; then
        echo "${LOOPS[0]}"
    else
        echo "ERROR: multiple loop devices are mapped to $IMAGE:"
        printf '%s\n' "${LOOPS[@]}"
        echo "Refusing to continue."
        exit 1
    fi
}

create() {
    echo "[+] Creating image (${SIZE_MB}MB)…"
    dd if=/dev/zero of="$IMAGE" bs=1M count="$SIZE_MB"

    echo "[+] Formatting as exFAT…"
    mkfs.exfat -n "$LABEL" "$IMAGE"

    echo "[+] Attaching loop..."
    LOOP=$(sudo losetup --find --show "$IMAGE")
    echo "[+] Loop: $LOOP"

    echo "[+] Waiting for auto-mount…"
    sleep 1

    echo "[+] Loop device: $LOOP"
    udisksctl mount -b "$LOOP"    

    MOUNT_PATH=$(find "/media/$USER" -maxdepth 1 -type d -name "$LABEL")
    echo "[+] Mounted at: $MOUNT_PATH"
}

unmount_img() {
    LOOP=$(get_loop)
    if [ -z "$LOOP" ]; then
        echo "[-] No loop device mapped to $IMAGE"
        exit 1
    fi
    echo "[+] Loop found: $LOOP"

    echo "[+] Unmounting via udisksctl…"
    udisksctl unmount -b "$LOOP"

    echo "[+] Detaching loop device…"
    sudo losetup -d "$LOOP"

    echo "[+] Fake device cleanly removed."
}

case "$1" in
    create) create ;;
    unmount) unmount_img ;;
    *)
        echo "Usage: $0 {create|unmount}"
        exit 1
        ;;
esac
