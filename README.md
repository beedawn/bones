# Bones
This is a Rust and Svelte web application that utilizes a PostgreSQL database to manage bills. It is still very much under development.


## Deployment
This application has been dockerized, and can be deployed via docker. The CORS permissions only allow it to be accessed from the same machine running the docker container, as it is no where near ready for production.

First you must have docker installed to install docker, see the link here:
https://docs.docker.com/engine/install/

Once Docker is installed, you will need to rename the example.env file to .env. For testing purposes, this configuation should be fine.

```bash
mv example.env .env
```

Then use docker compose to launch the docker containers
```bash
docker compose up --build
```

Once this has been completed, the front end will be visable in your browser at:
http://localhost:4173
