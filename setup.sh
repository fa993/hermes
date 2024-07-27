# Run as root
yum update -y
yum install git -y
git — version

yum install epel-release
yum update
yum install nginx
yum install nginx
systemctl start nginx

default_name=ec2-user
input_name=$1
name=${input_name:-$default_name}

echo "Granting permissions to $name"

chown -R $name /etc/nginx/default.d/
chown -R $name /etc/systemd/system/