


### step 0

```console
$ sudo chown -R root:root /home/bitdad/LMS-Server/backups && sudo chmod -R 777 /home/bitdad/LMS-Server/backups
```

### Step 1

```console
$ suso cp .pgpass /home/bitdad/
```

### Step 2

```console
$ sudo chown -R root:root /home/bitdad/.pgpass
```

### Step 3

```console
$ crontab backup
```