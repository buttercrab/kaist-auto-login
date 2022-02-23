FROM ubuntu:20.04

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y gnupg wget curl unzip ca-certificates --no-install-recommends

RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -

RUN echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update -y && \
    apt-get install -y google-chrome-stable && \
    rm -rf /var/lib/apt/lists/*

RUN CHROMEVER=$(google-chrome --product-version | grep -o "[^\.]*\.[^\.]*\.[^\.]*") && \
    DRIVERVER=$(curl -s "https://chromedriver.storage.googleapis.com/LATEST_RELEASE_$CHROMEVER") && \
    wget -q --continue -P /chromedriver "http://chromedriver.storage.googleapis.com/$DRIVERVER/chromedriver_linux64.zip"

RUN unzip /chromedriver/chromedriver* -d /chromedriver
EXPOSE 4444

ENTRYPOINT ["/chromedriver/chromedriver", "--port=4444"]
