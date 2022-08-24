


## Setup Ayoub APIs Reverse Proxy

### Install Nginx

```console
sudo apt install nginx && sudo apt install certbot python3-certbot-nginx
```

### Available Service Config File 

```console
sudo cp api.conf  /etc/nginx/conf.d/api.conf
```

## Setup SSL for APIs

```console
sudo systemctl restart nginx && sudo certbot --nginx
```

## NOTE

> Remember to enable `ufw` and all app ports.

> Remember to put the `api.conf` inside the `/etc/nginx/conf.d/api.conf` in order the CORS works fine through the nginx reverse proxy.