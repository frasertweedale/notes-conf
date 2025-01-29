# Streamlining bootable container workflows with podman-bootc

German Maglione

- Image Mode
  - an extension to OCI model
  - immutable, layering, registries, signing, CI/CD, GitOps
  - describe OS behaviour as prebuilt, predefined unit, the container
    image is *source of truth*
  - Build, deploy (VM with kickstart or bare metal via anaconda)

- boot-image-builder
  - bootc: install, upgrade (download new image and stage), switch,
    rollback

- podman-bootc
  - convenient tool that does all of the above
  - streamline the "edit-compile-debug" cycle for bootable
    containers

  - run (bootc container as VM), ssh (in), stop (VM), list
    (installed containers), rm (installed bootc VMs)


- Future plans
  - remove `podman machine` requirement on Linux
  - bootc image builder support
  - `krunkit` support on macOS: GPU sharing & better virtiofs spuport
  - Windows support
  - `convert`/`export` commands
  - Podman integration: `podman bootc`

- Other talks
  - Tomorrow keynote: "What if we could boot a container?"
