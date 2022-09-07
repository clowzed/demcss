apt-get -y install postfix
crontab -l | { echo "MAILTO=clowzed.work@gmail.com"; cat} | crontab -
crontab -l | {cat; echo "* * * * * sh /root/demcss/deploy.sh > /root/server.log"} | crontab -
