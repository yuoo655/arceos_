# test on qemu
# make A=apps/net/cvitek_nic_httpserver_test ARCH=aarch64 SMP=1 NET=y FS=y APP_FEATURES=use-ramdisk LOG=info run
make A=apps/net/cvitek_nic_test ARCH=aarch64 SMP=1 NET=y LOG=info run
# test on cv1811
# make A=apps/net/cvitek_nic_httpserver_test ARCH=riscv64 SMP=1 NET=y  FS=y APP_FEATURES=use-ramdisk LOG=info cv1811
# make A=apps/net/cvitek_nic_test ARCH=riscv64 SMP=1 NET=y LOG=info cv1811



# tftp ${uImage_addr} arceos-cv1811.itb
setenv ipaddr 192.168.1.254 
setenv serverip 192.168.1.104
tftpboot ${uImage_addr} 192.168.1.104:arceos-cv1811.itb
bootm ${uImage_addr}