# test on qemu
# make A=apps/net/cvitek_nic_httpserver_test ARCH=aarch64 SMP=1 NET=y FS=y APP_FEATURES=use-ramdisk LOG=info run
make A=apps/net/cvitek_nic_test ARCH=aarch64 SMP=1 NET=y LOG=info run
# test on cv1811
# make A=apps/net/cvitek_nic_httpserver_test ARCH=riscv64 SMP=1 NET=y  FS=y APP_FEATURES=use-ramdisk LOG=info cv1811
# make A=apps/net/cvitek_nic_test ARCH=riscv64 SMP=1 NET=y LOG=info cv1811
