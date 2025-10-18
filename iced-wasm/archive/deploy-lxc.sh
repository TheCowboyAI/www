#!/usr/bin/env bash
set -e

echo "=== Deploying NixOS WWW LXC with Proxmox workaround ==="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CONTAINER_ID="134"
PROXY_HOST="root@143.105.59.182"
PROXY_PORT="9922"
TEMPLATE_NAME="nixos-www-134.tar.xz"
CONTAINER_NAME="www"
CONTAINER_IP="10.0.64.134"
GATEWAY="10.0.64.1"
PREFIX="24"

echo -e "${YELLOW}Configuration:${NC}"
echo "  Container ID: $CONTAINER_ID"
echo "  Container Name: $CONTAINER_NAME"
echo "  Container IP: $CONTAINER_IP/$PREFIX"
echo "  Gateway: $GATEWAY"
echo "  Proxmox Host: $PROXY_HOST:$PROXY_PORT"
echo ""

echo -e "${YELLOW}Step 1: Building LXC container...${NC}"
nix build .#packages.x86_64-linux.wwwlxc

echo -e "${YELLOW}Step 2: Creating fixed tarball...${NC}"
# Extract and fix the tarball to include /root directory
TEMP_DIR=$(mktemp -d)
echo "Extracting to temporary directory: $TEMP_DIR"

# Extract original tarball
tar -xf result/tarball/*.tar.xz -C "$TEMP_DIR"

# Create the missing /root directory
mkdir -p "$TEMP_DIR/root"

# Create a basic .bashrc for root
cat > "$TEMP_DIR/root/.bashrc" << 'EOF'
# NixOS container root bashrc
export PS1='\[\033[01;31m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '
export PATH=/run/current-system/sw/bin:$PATH
EOF

# Repackage with the fix
echo "Creating fixed tarball..."
cd "$TEMP_DIR"
tar -czf "/tmp/${TEMPLATE_NAME}" .
cd - > /dev/null

# Cleanup temp directory
rm -rf "$TEMP_DIR"

echo -e "${YELLOW}Step 3: Uploading fixed container to Proxmox...${NC}"
scp -P $PROXY_PORT -i ~/.ssh/id_cim_thecowboyai "/tmp/${TEMPLATE_NAME}" "${PROXY_HOST}:/var/lib/vz/template/cache/${TEMPLATE_NAME}"

echo -e "${YELLOW}Step 4: Creating LXC container on Proxmox...${NC}"
# Create container using CLI to avoid the UI issue
ssh -p $PROXY_PORT -i ~/.ssh/id_cim_thecowboyai "$PROXY_HOST" << EOF
# Stop and destroy container if it exists
if pct status $CONTAINER_ID >/dev/null 2>&1; then
    echo "Stopping existing container $CONTAINER_ID..."
    pct stop $CONTAINER_ID || true
    sleep 2
    echo "Destroying existing container $CONTAINER_ID..."
    pct destroy $CONTAINER_ID --purge
fi

# Create new container
echo "Creating new container..."
pct create $CONTAINER_ID /var/lib/vz/template/cache/${TEMPLATE_NAME} \
  --hostname $CONTAINER_NAME \
  --memory 2048 \
  --cores 2 \
  --net0 name=eth0,bridge=vmbr1,ip=${CONTAINER_IP}/${PREFIX},gw=${GATEWAY} \
  --storage local-lvm \
  --rootfs local-lvm:8 \
  --unprivileged 1 \
  --features nesting=1,keyctl=1 \
  --ostype unmanaged \
  --cmode console \
  --onboot 1

# Add init command
echo 'lxc.init.cmd: /sbin/init' >> /etc/pve/lxc/${CONTAINER_ID}.conf
EOF

echo -e "${YELLOW}Step 5: Starting container...${NC}"
ssh -p $PROXY_PORT -i ~/.ssh/id_cim_thecowboyai "$PROXY_HOST" "pct start $CONTAINER_ID"

echo -e "${YELLOW}Step 6: Waiting for container to boot...${NC}"
sleep 15

echo -e "${YELLOW}Step 7: Verifying container...${NC}"
ssh -p $PROXY_PORT -i ~/.ssh/id_cim_thecowboyai "$PROXY_HOST" "pct exec $CONTAINER_ID -- /run/current-system/sw/bin/uname -a" || {
    echo -e "${RED}Container verification failed. Checking status...${NC}"
    ssh -p $PROXY_PORT -i ~/.ssh/id_cim_thecowboyai "$PROXY_HOST" "pct status $CONTAINER_ID"
    echo -e "${YELLOW}Checking if nginx is running...${NC}"
    ssh -p $PROXY_PORT -i ~/.ssh/id_cim_thecowboyai "$PROXY_HOST" "pct exec $CONTAINER_ID -- /run/current-system/sw/bin/systemctl status nginx || echo 'Nginx status check failed'"
}

# Cleanup local temp file
rm -f "/tmp/${TEMPLATE_NAME}"

echo -e "${GREEN}=== Deployment complete! ===${NC}"
echo -e "${GREEN}Container $CONTAINER_ID created successfully${NC}"
echo ""
echo "Next steps:"
echo "1. Test site: curl https://thecowboy.ai"
echo "2. Check container logs: ssh -p $PROXY_PORT -i ~/.ssh/id_cim_thecowboyai $PROXY_HOST 'pct console $CONTAINER_ID'"