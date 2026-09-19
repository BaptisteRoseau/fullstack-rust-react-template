# Configs

This directory contains runtime-mounted configurations for the services.

If the configuration should instead be always the same, embed it in directly in the docker image.

## Convention

This directory contains directories for each configured service, named by the snake-case service name (ex: `keycloak`, `prometheus`).

If a service configuration needs a difference between environments (`dev`, `staging`, `prod`, ..), split it like `<service>/<environment>`:

```txt
└── service
    └── dev
        └── myconfig.yml
    └── prod
        └── myconfig.yml
```

If a config is the same across all environments, or used by only one of them, do not split by environment and include it directly:

```txt
└── service
    └── myconfig.yml
```

## Security

**Never commit secrets in the config files**.

Load them at runtime through a secret manager or environment variables.
