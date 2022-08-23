


## Setup Ayoub APIs Reverse Proxy

### Install Nginx

```console
sudo apt install nginx && sudo apt install certbot python3-certbot-nginx
```

### Available Service Config File 

```console
sudo cp api.ayoub.com.conf /etc/nginx/sites-available/api.ayoub.com.conf
```

### Enable Service Config File 

```console
sudo ln -s /etc/nginx/sites-available/api.ayoub.com.conf /etc/nginx/sites-enabled/api.ayoub.com.conf
```

## Setup SSL for APIs

```console
sudo systemctl restart nginx && sudo certbot --nginx
```