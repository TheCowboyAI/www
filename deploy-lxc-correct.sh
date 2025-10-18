#!/usr/bin/env bash

# IMPORTANT: SSL certificates must be deployed separately!
# This script does NOT include certificates for security reasons.
# 
# Before running this script, manually copy certificates to the server:
#   scp -P 9922 -i ~/.ssh/id_cim_thecowboyai certs/thecowboy.ai.pem root@143.105.59.182:/tmp/
#   scp -P 9922 -i ~/.ssh/id_cim_thecowboyai certs/thecowboy.ai.key root@143.105.59.182:/tmp/
# 
# Then after container creation, copy them into the container:
#   pct push 134 /tmp/thecowboy.ai.pem /etc/ssl/certs/thecowboy.ai.pem
#   pct push 134 /tmp/thecowboy.ai.key /etc/ssl/private/thecowboy.ai.key
#   pct exec 134 -- chmod 644 /etc/ssl/certs/thecowboy.ai.pem
#   pct exec 134 -- chmod 600 /etc/ssl/private/thecowboy.ai.key
#   pct exec 134 -- mkdir -p /etc/ssl/private
#   rm /tmp/thecowboy.ai.*
set -e

echo "=== Deploying NixOS WWW LXC Container (Correct Method) ==="

# Configuration
CONTAINER_ID="134"
PROXY_HOST="143.105.59.182"
PROXY_PORT="9922"
SSH_KEY="~/.ssh/id_cim_thecowboyai"
TEMPLATE_STORAGE='local'
TEMPLATE_FILE='nixos-www-134-fixed.tar.xz'
CONTAINER_HOSTNAME='www'
CONTAINER_STORAGE='local-lvm'
CONTAINER_RAM_IN_MB='2048'
CONTAINER_DISK_SIZE_IN_GB='8'
CONTAINER_IP='10.0.64.134/19'
GATEWAY='10.0.64.1'
VLAN_TAG='64'

echo "Building LXC container..."
nix build .#packages.x86_64-linux.wwwlxc

echo "Uploading template to Proxmox..."
scp -C -P $PROXY_PORT -i $SSH_KEY \
  result/tarball/*.tar.xz \
  "root@${PROXY_HOST}:/var/lib/vz/template/cache/${TEMPLATE_FILE}"

echo "Creating container on Proxmox..."
ssh -p $PROXY_PORT -i $SSH_KEY root@$PROXY_HOST << EOF
# Stop and destroy container if it exists
if pct status $CONTAINER_ID >/dev/null 2>&1; then
    echo "Stopping existing container $CONTAINER_ID..."
    pct stop $CONTAINER_ID || true
    sleep 2
    echo "Destroying existing container $CONTAINER_ID..."
    pct destroy $CONTAINER_ID --purge
fi

# Create new container using the CRITICAL parameters from the guide
pct create "$CONTAINER_ID" \
  "${TEMPLATE_STORAGE}:vztmpl/${TEMPLATE_FILE}" \
  --arch amd64 \
  --ostype unmanaged \
  --description "NixOS WWW LXC Container" \
  --hostname "${CONTAINER_HOSTNAME}" \
  --net0 name=eth0,bridge=vmbr0,ip=${CONTAINER_IP},gw=${GATEWAY},tag=${VLAN_TAG},firewall=1 \
  --storage "${CONTAINER_STORAGE}" \
  --memory "${CONTAINER_RAM_IN_MB}" \
  --rootfs ${CONTAINER_STORAGE}:${CONTAINER_DISK_SIZE_IN_GB} \
  --unprivileged 1 \
  --features nesting=1 \
  --cmode console \
  --onboot 1 \
  --start 1

echo "Container created and started!"
EOF

echo "Waiting for container to boot..."
sleep 20

echo "Testing container..."
ssh -p $PROXY_PORT -i $SSH_KEY root@$PROXY_HOST \
  "pct exec $CONTAINER_ID -- /run/current-system/sw/bin/uname -a" || \
  echo "Note: First access may fail, this is normal"

echo "=== Deployment Complete ==="
echo ""
echo "Container $CONTAINER_ID should now be running NixOS with nginx serving the WASM site"
echo "Test with: curl https://thecowboy.ai"