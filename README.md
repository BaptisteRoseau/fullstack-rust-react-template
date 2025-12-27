# Rust React Template

**WARNING**: This project is under a heavy refactor right now and is completely broken.

This project is a work in progress web application template using low-level open source software.

It is designed to be secure, cloud native, very low CPU and memory footprint and handle hundreds of users in parallel even from a 30$ VPS.

It is currently in a very early stage.

When finished, its goal is for a developer or company to be able to clone the project, implement only their frontend and backend using React Router and Axum, follow a short tutorial on how to deploy and then have an application able to handle a huge load for the fraction of the cost required when using a cloud platform.

## TODOs

### Authentication

Authentication will be done by the most performant open source software available:

- Authelia (API Gateway)
- OpenLDAP (LDAP DB for Authelia)
- NGINX (Reverse Proxy)

- [ ] Docker Compose
    - [ ] Authelia
    - [ ] NGINX
    - [ ] OpenLDAP
- [ ] Configurations
    - [ ] Authelia
    - [ ] NGINX
    - [ ] OpenLDAP
- [ ] App integration
    - [ ] Register/Login form
    - [ ] JWT storage

### Payment

- [ ] Stripe integration

### Database

- [ ] PostgreSQL

### CI/CD

Make scripts, then call them from CI.

- [ ] Build Packages
- [ ] Run test scripts
- [ ] GitLab
- [ ] GitHub
- [ ] Deployment tutorial/script
- [ ] Nix Shell config for executing locally without Docker

### Frontend

- [ ] Home Page
- [ ] User Panel
- [ ] Routing
- [ ] Automate robots.txt
- [ ] Automate sitemap.xml

### Backend

- [ ] Graceful Shutdown (currently gracefully cannot stop from container)
- [ ] Authentication integration

### Other

- [X] build scripts
- [X] test scripts
- [X] Environment variables in docker compose and .env
- [ ] fix files permission

### Documentation

- [ ] Introduction (what is this project?)
- [ ] Repo organization
- [ ] Backend
- [ ] Frontend
- [ ] How to run
- [ ] How to build
- [ ] How to test
- [ ] How to develop
- [ ] How to deploy
- [ ] Build HTML documentation from markdown files
