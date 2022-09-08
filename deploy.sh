cd /root

rm -rf ./demcss
rm ./demcss

# Cloning project
git clone https://github.com/clowzed/demcss

# Installing rust without confirmation
curl https://sh.rustup.rs -sSf | sh -s -- -y

cd demcss

. "$HOME/.cargo/env"

# Building application
cargo build --release

# Moving executable back in order to remove target folder (too big size)
mv ./target/release/demcss ./

# Removing target folder
rm -rf ./target

# Installing nginx
# We think that everything is ok
apt-get -y install nginx

# Copy nginx configuration and link to enabled sites
cat ./nginx.conf > /etc/nginx/sites-available/demcss
ln /etc/nginx/sites-available/demcss /etc/nginx/sites-enabled

# Enable ufw
yes | ufw enable


# Allow ports 
yes | ufw allow 80 
yes | ufw allow 443
yes | ufw allow 22

# Starting proxy
yes | systemctl restart nginx

# Killing all listners of 8080
kill $(lsof -t -i:8080)

# Running
./demcss > /root/server.log