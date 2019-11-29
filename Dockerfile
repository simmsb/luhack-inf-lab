FROM docker/compose

WORKDIR /opt/luhack-inf-lab
COPY . .
COPY .env .env

CMD docker-compose up
