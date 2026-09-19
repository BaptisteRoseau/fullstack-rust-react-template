# Kubernetes

Manifests to deploy the application on a Kubernetes cluster: the backend, the frontend and the
services they depend on (Postgres, Redis, SeaweedFS, Keycloak, Prometheus, Grafana).

Docker Compose remains the way to run the stack locally, see
[../docker-compose](../docker-compose/README.md). This directory targets real clusters.

## Layout

Manifests are grouped **by component**, not by kind: one directory per deployable thing, holding
every object that thing needs. Components are gathered in the same groups the Compose files use,
so a profile there maps to a directory here. Environment differences live in Kustomize overlays
that patch the base, so a component is described once.

```text
kubernetes/
├── base/                           # The services required to run the application
│   ├── backend/
│   │   ├── configmap.yaml
│   │   ├── deployment.yaml
│   │   ├── hpa.yaml
│   │   ├── pdb.yaml
│   │   ├── secret.example.yaml
│   │   ├── service.yaml
│   │   └── kustomization.yaml
│   ├── frontend/
│   │   ├── deployment.yaml
│   │   ├── hpa.yaml
│   │   ├── ingress.yaml
│   │   ├── pdb.yaml
│   │   ├── service.yaml
│   │   └── kustomization.yaml
│   ├── postgres/
│   │   ├── configmap.yaml
│   │   ├── secret.example.yaml
│   │   ├── service.yaml
│   │   ├── statefulset.yaml
│   │   └── kustomization.yaml
│   ├── keycloak/
│   ├── keycloak-postgres/
│   ├── migrate/
│   ├── redis/
│   ├── seaweedfs/
│   └── kustomization.yaml          # lists the components above
├── monitoring/                     # Prometheus, Grafana, Postgres Exporter
│   ├── grafana/
│   ├── postgres-exporter/
│   ├── prometheus/
│   └── kustomization.yaml
├── debug/                          # Development-only conveniences
│   ├── homepage/
│   ├── mailhog/
│   └── kustomization.yaml
└── overlays/                        # Same component tree as the groups they patch
    ├── dev/
    │   ├── backend/
    │   │   ├── configmap.yaml
    │   │   └── deployment.yaml
    │   ├── frontend/
    │   │   └── ingress.yaml
    │   ├── homepage/
    │   │   └── configmap.yaml
    │   ├── keycloak/
    │   │   └── ingress.yaml
    │   ├── namespace.yaml
    │   └── kustomization.yaml      # namespace, image tags, replicas: 1, patch list
    └── production/
        ├── backend/
        │   ├── configmap.yaml
        │   ├── deployment.yaml
        │   └── hpa.yaml
        ├── frontend/
        │   └── ingress.yaml
        ├── keycloak/
        │   ├── configmap.yaml
        │   ├── deployment.yaml
        │   └── ingress.yaml
        ├── namespace.yaml
        └── kustomization.yaml
```

## Conventions

- **One directory per component** under a group, named after the service it deploys: `backend/`,
  `frontend/`, `postgres/`.
- **One object per file, named after its kind**, in the singular: `deployment.yaml`,
  `service.yaml`, `configmap.yaml`, `hpa.yaml`, `pdb.yaml`, `ingress.yaml`. The directory already
  carries the component name — never repeat it in the file name.
- **Abbreviate long kinds** the way `kubectl` does: `hpa.yaml` not `horizontalpodautoscaler.yaml`,
  `pdb.yaml` not `poddisruptionbudget.yaml`.
- **`metadata.name` is the component name**, so an object found with `kubectl get` maps back to the
  directory it came from.
- **Every component directory has a `kustomization.yaml`** listing its own resources. A group's
  `kustomization.yaml` lists its components; nothing else enumerates files.
- **An overlay repeats the tree of what it patches.** A patch lives in a directory named after the
  component it targets, in a file named after the kind it patches — `backend/configmap.yaml`
  patches the ConfigMap that `base/backend/configmap.yaml` declares — so a component is found at
  the same path in the base, in `monitoring/`, in `debug/` and in every overlay. Objects that
  belong to no component, such as `namespace.yaml`, stay at the overlay root.
- **Overlay component directories have no `kustomization.yaml`.** Patches are not resources: the
  overlay's own `kustomization.yaml` lists them under `patches:`, and a nested kustomization would
  apply them a second time as objects of their own.
- **Bases stay environment-agnostic.** No namespace, no image tag, no replica count, no host name —
  overlays set those. Anything that differs between dev and production is a patch.
- **Label everything** with `app.kubernetes.io/name`, `instance`, `component`, `part-of` and
  `version` so selectors, dashboards and `kubectl get all -l` work. Bases carry `name`, `component`
  and `part-of`; overlays add `instance`, and `version` follows the image tag they set.
- **Selectors match on `app.kubernetes.io/name` only.** A selector is immutable once applied, so it
  must not pick up a label an overlay can change.
- **Configuration is never copied.** The migrations, the Keycloak realm and the Homepage dashboard
  are read from their canonical location through `configMapGenerator`, the same files Docker
  Compose mounts.
- **Every workload declares probes, resource requests and a restrictive `securityContext`**
  (`runAsNonRoot`, `allowPrivilegeEscalation: false`, all capabilities dropped, and a read-only
  root filesystem wherever the image tolerates one).
- **Services present once on every node belong to the node**, like node-exporter or Prometheus, do not add them as kubernetes containers but add them in the [NixOS](../nix) image.

## Environments

Two overlays:

| Overlay              | Target                | Typical patches                                            |
| -------------------- | --------------------- | ---------------------------------------------------------- |
| `overlays/dev`       | Shared dev cluster    | Single replicas, small resource requests, debug log level, no HPA, the `debug` group |
| `overlays/production`| Production cluster    | HPAs, PDBs, production hosts and TLS, tuned resources, Keycloak in production mode |

Building needs `--load-restrictor LoadRestrictionsNone`, because the generated ConfigMaps read
their content from outside this directory rather than duplicating it:

```bash
kustomize build --load-restrictor LoadRestrictionsNone infrastructure/kubernetes/overlays/dev |
    kubectl apply -f -

kustomize build --load-restrictor LoadRestrictionsNone infrastructure/kubernetes/overlays/production |
    kubectl apply -f -

# Preview without touching the cluster
kustomize build --load-restrictor LoadRestrictionsNone infrastructure/kubernetes/overlays/production
```

Image tags are set by the overlays through `images:` and must be immutable — a git SHA, never
`latest`. A release is therefore a one-line change to an overlay.

Namespaces, CRDs and the cluster-scoped objects a component assumes are applied before it, on a
first install only. The example hosts (`app.example.com`, `auth.example.com`) and image registry
(`ghcr.io/example`) are placeholders: replace them with the cluster's own before the first apply.

The `migrate` Job runs the SQLx migrations once and exits. A Job's pod template is immutable, so
adding a migration means deleting the previous Job before re-applying:

```bash
kubectl delete job migrate --ignore-not-found
```

## Secrets

Secrets are **never** committed. `Secret` manifests are base64, not encryption.

Each component directory may hold a `secret.example.yaml` documenting the keys it expects, with
empty values. Those files are deliberately absent from the `kustomization.yaml` files: they are
documentation, not resources. Real values come from the cluster's secret manager — External
Secrets Operator, Sealed Secrets or SOPS, whichever the target cluster uses — published under the
same name as the component that consumes them.

Prefer mounting secrets as files over injecting them as environment variables, which leak into
crash dumps and child processes.

## Validation

```bash
for overlay in dev production; do
    kustomize build --load-restrictor LoadRestrictionsNone \
        "infrastructure/kubernetes/overlays/$overlay" |
        kubeconform -summary -strict -kubernetes-version 1.30.0
done
```

## Rules

- This directory is part of the `infrastructure` and does not know about the rest of the repository.
- It can only use files from:
    - [infrastructure/kubernetes](./infrastructure/kubernetes)
    - [infrastructure/configs](./infrastructure/configs)
    - [infrastructure/docker](./infrastructure/docker) (only for the image names)
- Volumes are considered either empty or filled with a config map. Never bind code or config or relative path.
- Always pin exact versions to container images, never `latest`.
