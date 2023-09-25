# Rust Web Service Template

This repository contains a template for setting up Rust Web Development. 
To handle database migrations, I have opted for Django due to its convenient 
built-in functionality. For database ORM, I have used diesel. To facilitate 
development, I have configured the CLI using zsh, which has been my go-to shell 
for some time. For the HTTP service, I have chosen hyper due to its lightweight nature.


## Rust Setup

### Install Rust

- https://www.rust-lang.org/tools/install

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## IDE Setup

I have opted for PyCharm Community Edition for updating the Django Models and 
IntelliJ IDEA Community Edition for Rust.


## Zsh Setup

### Install `zsh`

### For Mac

```shell

# Installing the zsh
brew install zsh

# Setting the `zsh` as your default shell
chsh -s /bin/zsh

# After that restart your terminal, now you should be using `zsh` as your default shell
```

#### For Ubuntu and Debian-based Linux distributions


```shell

# Installing the zsh
sudo apt install zsh

# Setting the `zsh` as your default shell 
chsh -s $(which zsh)

# After that restart your terminal, now you should be using `zsh` as your default shell
```


### Autoenv Setup

```shell
mkdir ~/.dotfiles
git clone https://github.com/Tarrasch/zsh-autoenv ~/.dotfiles/lib/zsh-autoenv
echo 'source ~/.dotfiles/lib/zsh-autoenv/autoenv.zsh' >> ~/.zshrc
```
- Reference: https://github.com/Tarrasch/zsh-autoenv


## Django Setup

### Install `pip`

### Create `virtualenv`

```shell
python3 -m venv .env
source ./.env/bin/activate
```

### Install `python` Dependencies

```shell
pip install -r requirements.txt
```

### Create Django Project

We don't need to run this command everytime as we have to create only first time, it is already done

```shell
django-admin startproject proj
mv proj dj
```

### Create Django APP

```shell
cd dj
django-admin startapp temp_app
```

Add this APP into dj/proj/settings.py in `INSTALLED_APPS`.

```python
INSTALLED_APPS = [
    "django.contrib.admin",
    "django.contrib.auth",
    "django.contrib.contenttypes",
    "django.contrib.sessions",
    "django.contrib.messages",
    "django.contrib.staticfiles",
    "temp_app"
]
```

## Install Postgres


## Install `Redis`


## Diesel Setup

### Install Diesel CLI

If above `zsh` setup is not done

```shell
cargo install diesel_cli --no-default-features --features "postgres"
```

If above `zsh` is done so run below command

```shell
install_diesel
```

### Diesel Schema

If above `zsh` setup is not done
```shell
diesel print-schema only-tables --database-url=$DATABASE_URL > $PROJDIR/service/db/src/schema.rs
```

If above `zsh` is done so run below command

```shell
diesel_schema
```
