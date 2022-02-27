#!/usr/bin/env python3

import random


def main():
    password_charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~"
    password = "".join([random.choice(password_charset) for i in range(10)])

    with open("docker-compose-template.yaml", "r") as f:
        config = f.read()

    config = config.replace("%POSTGRES_PASSWORD%", f'"{password}"')
    imap_domain = input("enter imap domain: ")
    config = config.replace("%IMAP_DOMAIN%", f'"{imap_domain}"')
    mailcow_url = input("enter mailcow url: ")
    config = config.replace("%MAILCOW_URL%", f'"{mailcow_url}"')
    mailcow_api_key = input("enter mailcow api key: ")
    config = config.replace("%MAILCOW_API_KEY%", f'"{mailcow_api_key}"')
    new_email_domain = input("enter new email domain: ")
    config = config.replace("%NEW_EMAIL_DOMAIN%", f'"{new_email_domain}"')
    port = int(input("enter port to serve: "))
    config = config.replace("%PORT%", str(port))

    with open("docker-compose.yaml", "w") as f:
        f.write(config)

    print("Configuration is written on docker-compose.yaml")


if __name__ == '__main__':
    main()
