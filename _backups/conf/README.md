


## Setup Ayoub APIs Reverse Proxy

### Install Nginx

```console
sudo apt install nginx
```

### Available Service Config File 

```console
sudo cp api.auth.ayoub.com.conf /etc/nginx/sites-available/api.auth.ayoub.com.conf && sudo cp api.event.ayoub.com.conf /etc/nginx/sites-available/api.event.ayoub.com.conf && sudo cp api.game.ayoub.com.conf /etc/nginx/sites-available/api.game.ayoub.com.conf
```

### Enable Service Config File 

```console
sudo ln -s /etc/nginx/sites-available/api.auth.ayoub.com.conf /etc/nginx/sites-enabled/api.auth.ayoub.com.conf && sudo ln -s /etc/nginx/sites-available/api.event.ayoub.com.conf /etc/nginx/sites-enabled/api.event.ayoub.com.conf && sudo ln -s /etc/nginx/sites-available/api.game.ayoub.com.conf /etc/nginx/sites-enabled/api.game.ayoub.com.conf
```

## Setup API SSL

```console
sudo systemctl restart nginx && sudo certbot --nginx
```