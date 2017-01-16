Turtles all the way down - thin lvm + kvm tips and tricks
=========================================================

- how do you deal with TRIM / discard when you have lots of layers?
- turtles and cows

- SSD + Trim / Unmap / Discard
  - pass down delete requests to underlying flash storage
  - OS + FS need to support TRIM

- SSD > partition > encryption > fs
- ssd > part > fde > lvm > (thinpool) > kvm > (thinpool)
  > nested kvm > virtual disk > partition > lvm > filesystem

- lvm passthru
  - in ``/etc/lvm/lvm.conf``: ``issue_discards = 1``

- dmcrypt / LUKS
  - ``/etc/crypttab``
    - luks-blah-blah UUID=blah-blah none luks,discard
  - dracut -force
  - blog by christopher smart

- Thin LVM
  - lvcreate -T -L 80G myVG/lv_thin

- Trim + libvirt
  - check machine type > 2.1
  - use virtio-scsi driver (as default)
  - add 'discard' to hard disks

- on VM
  - treat like a physical env

- Are you Trim?
  - boot and test VM
  - ``lsblk -o MOUNTPOINT,DISC-MAX,FSTYPE``
  - ``fstrim -v <mountpoint>``

- Fat -> Thin
  - ``virt-sparsify --in-place /dev/myVG/myVMDisk``
  - ``virt-sparsify myVMDisk.img /dev/myVG/myVMDisk``
  - use ``lvs`` to check

- Thin -> Thin
  - ``lzop`` (compression / decompression)

- Nesting turtles
  - /etc/modprobe.d/kvm-intel.conf
    - options kvm-intel nested=1

- Raid and Trim?
  - two key software raid technologies: ``mdadm``, ``lvm raid``

- KVM tips
  - trim/discard with libvirt + KVM
  - virt-sparsify
  - nesting
  - enable KVM serial console

- KVM serial console
  - modify /etc/default/grub
    - GRUB_TERMINAL="serial console"
    - GRUB_SERIAL_COMMAND="serial --speed=38400 --unit=0 --word=8
      --parity=no -stop=1
    - GRUB_CMDLINE_LINUX
      - remove "rhgb quiet"
      - add "console=ttyS0,38400n8"

- ansible playbook: steven-ellis/ansible-playpen : grub_console.yaml
  - currently RHEL/CentOS/Fedora-centric
