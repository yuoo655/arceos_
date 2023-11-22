# 前置环境搭配
见 https://github.com/orgs/rcore-os/discussions/24
# 此文件夹使用
直接执行mkitb.cv1811h.sh脚本即可自动生成.itb文件并且mv到srv/tftp/文件夹下  
常见问题：mkimage执行失败：下载u-boot-tools工具即可
# 华山派上操作
1. 配置ip地址，保持与tftp服务器在一个网段
setenv ipaddr xxx.xxx.xxx.xxx
2. 加载itb文件到指定位置
tftpboot 0x82000000 arceos-cv1811h.itb
3. 跳转到指定位置并启动内核
bootm 0x82000000

详细步骤可以参考上方链接，有疑问或者更好的想法可与我们交流
