curl https://github.com/naoki-tomita/oci-agent/releases/download/main/aarch64-binary -L -o oci-agent
chmod +x oci-agent
sudo mv oci-agent /usr/local/bin
touch ~/.oci-config