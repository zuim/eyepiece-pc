# Goggles device dump

A raw snapshot of one DJI Goggles 3 unit (firmware 01.00.1600) taken over `adb`: mounts, properties, processes,
devices, partitions, logs. It is a reference for what the system looks like, not a specification, and is not
kept up to date.


## Mountpoints

```shell
$ mount
rootfs on / type rootfs (ro,seclabel,size=224588k,nr_inodes=56147)
devtmpfs on /dev type devtmpfs (rw,seclabel,nosuid,relatime,size=224588k,nr_inodes=56147,mode=755)
devpts on /dev/pts type devpts (rw,seclabel,relatime,mode=600,ptmxmode=000)
proc on /proc type proc (rw,relatime,gid=3009,hidepid=2)
sysfs on /sys type sysfs (rw,seclabel,relatime)
selinuxfs on /sys/fs/selinux type selinuxfs (rw,relatime)
tmpfs on /mnt type tmpfs (rw,seclabel,nosuid,nodev,noexec,relatime,size=229928k,nr_inodes=57482,mode=755,gid=1000)
/dev/block/dm-0 on /system type ext4 (ro,seclabel,relatime)
/dev/block/mmcblk0p15 on /vendor type ext4 (ro,seclabel,relatime)
none on /dev/cpuctl type cgroup (rw,nosuid,nodev,noexec,relatime,cpu)
none on /acct type cgroup (rw,nosuid,nodev,noexec,relatime,cpuacct)
none on /dev/memcg type cgroup (rw,nosuid,nodev,noexec,relatime,memory)
/dev/block/mmcblk0p22 on /data type ext4 (rw,seclabel,nosuid,relatime,discard,nodelalloc,data=ordered)
/dev/block/mmcblk0p21 on /blackbox type ext4 (rw,seclabel,relatime,discard,data=ordered)
tmpfs on /tmp type tmpfs (rw,seclabel,relatime,size=32768k,nr_inodes=57482)
none on /config type configfs (rw,nosuid,nodev,noexec,relatime)
/dev/block/mmcblk0p5 on /cali type ext4 (rw,seclabel,relatime,discard,data=ordered)
none on /dev/cpuset type cgroup (rw,nosuid,nodev,noexec,relatime,cpuset,noprefix,release_agent=/sbin/cpuset_release_agent)
cg2_bpf on /dev/cg2_bpf type cgroup2 (rw,nosuid,nodev,noexec,relatime)
pstore on /sys/fs/pstore type pstore (rw,seclabel,nosuid,nodev,noexec,relatime)
/sys/kernel/debug on /sys/kernel/debug type debugfs (rw,seclabel,relatime,mode=755)
/dev/block/mmcblk0p20 on /cache type ext4 (rw,seclabel,relatime,discard,data=ordered)
/dev/block/mmcblk0p4 on /factory_data type ext4 (rw,seclabel,relatime,discard,data=ordered)
/dev/block/mmcblk0p23 on /blackbox/upgrade type ext4 (rw,seclabel,relatime,discard,data=ordered)
none on /sys/kernel/config type configfs (rw,relatime)
adb on /dev/usb-ffs/adb type functionfs (rw,relatime)
bulk on /dev/usb-ffs/bulk type functionfs (rw,relatime)
bulk4 on /dev/usb-ffs/bulk4 type functionfs (rw,relatime)
bulk5 on /dev/usb-ffs/bulk5 type functionfs (rw,relatime)
bulk6 on /dev/usb-ffs/bulk6 type functionfs (rw,relatime)
bulk1 on /dev/usb-ffs/bulk1 type functionfs (rw,relatime)
/dev/block/mmcblk0p24 on /mnt/media_rw/emulated type exfat (rw,dirsync,nosuid,nodev,noexec,noatime,uid=1023,gid=1023,fmask=0007,dmask=0007,allow_utime=0020,iocharset=utf8,errors=remount-ro)
/dev/block/vold/sdcard0 on /mnt/media_rw/sdcard0 type exfat (rw,dirsync,nosuid,nodev,noexec,noatime,uid=1023,gid=1023,fmask=0007,dmask=0007,allow_utime=0020,iocharset=utf8,errors=remount-ro)
```

## Properties

```shell
$ getprop
[EnablePerfStats]: [true]
[camera_service.boot_times]: [0]
[camera_service.started_complete]: [1064]
[coredump_monitor.enable]: [1]
[dalvik.vm.isa.arm64.features]: [default]
[dalvik.vm.isa.arm64.variant]: [cortex-a55]
[dalvik.vm.lockprof.threshold]: [500]
[dalvik.vm.stack-trace-dir]: [/data/anr]
[dds_agent_reboot_times]: [0]
[debug.atrace.tags.enableflags]: [0]
[dji.agent_service]: [1]
[dji.amt_service]: [1]
[dji.arhome_service]: [1]
[dji.audio_service]: [1]
[dji.bb_log_exporter.downloading]: [0]
[dji.blackbox_service_v2]: [1]
[dji.camera_service]: [1]
[dji.camera_upgrade_service]: [1]
[dji.charger_service]: [1]
[dji.check_tombstone.completed]: [true]
[dji.config_store]: [1]
[dji.debug.lcd_mipi_err]: [0]
[dji.debug.lcd_recover]: [disable]
[dji.dsp_service]: [1]
[dji.fcdump_service]: [1]
[dji.ftpd_service]: [1]
[dji.gfsk_agent_service]: [1]
[dji.glasses_service]: [1]
[dji.goggles.mode]: [released]
[dji.interconnect_service]: [1]
[dji.log_store.stop]: [0]
[dji.lte_service]: [1]
[dji.media_player]: [1]
[dji.media_server]: [1]
[dji.network_service]: [1]
[dji.omx.enc.out.allocator.flag]: [true]
[dji.omx.log.interval]: [500]
[dji.player.instances]: [1]
[dji.player.version]: [BuildAt:2026-10-19 15:13:09,ChangeId: Ibc9dd9a2d1073c17421ea89ae92f775ae9eb408c]
[dji.prop.device_upgrade_status]: [idle]
[dji.prop.upgrade_status]: [idle]
[dji.recorder.version]: [OMX Release at 2026-10-19 15:13:09 VER: Ibc9dd9a2d1073c17421ea89ae92f775ae9eb408]
[dji.sdrs_agent_service]: [1]
[dji.sdrs_agent_service_startup]: [1]
[dji.sec.amt.init.sync]: [1]
[dji.sec_service]: [1]
[dji.sw_uav_service]: [1]
[dji.system_service]: [1]
[dji.upgrade_service]: [1]
[dji.wlm_service]: [1]
[init.svc.adbd]: [running]
[init.svc.cam_logcat]: [running]
[init.svc.console]: [running]
[init.svc.coredump_monitor]: [running]
[init.svc.dji_agent]: [running]
[init.svc.dji_amt]: [running]
[init.svc.dji_arhome]: [running]
[init.svc.dji_audio]: [running]
[init.svc.dji_blackbox_v2]: [running]
[init.svc.dji_camera3]: [running]
[init.svc.dji_charger]: [running]
[init.svc.dji_config_store]: [running]
[init.svc.dji_dsp]: [running]
[init.svc.dji_ftpd]: [running]
[init.svc.dji_gfsk_agent]: [running]
[init.svc.dji_glasses]: [stopped]
[init.svc.dji_media_player]: [running]
[init.svc.dji_media_server]: [running]
[init.svc.dji_network]: [running]
[init.svc.dji_sdrs_agent]: [running]
[init.svc.dji_sec]: [running]
[init.svc.dji_sw_uav]: [running]
[init.svc.dji_sys]: [running]
[init.svc.dji_upgrade]: [running]
[init.svc.dji_wlm]: [running]
[init.svc.init_part]: [stopped]
[init.svc.interconnect_service]: [running]
[init.svc.kernel_log]: [running]
[init.svc.logd]: [running]
[init.svc.logd-reinit]: [stopped]
[init.svc.netd]: [running]
[init.svc.servicemanager]: [running]
[init.svc.setup_usb]: [stopped]
[init.svc.start_dji_system]: [stopped]
[init.svc.sys_perf_log]: [running]
[init.svc.system_log]: [running]
[init.svc.tombstoned]: [running]
[init.svc.upgrade_log]: [running]
[init.svc.usb_bulk_raw]: [running]
[init.svc.usb_bulk_raw_01]: [running]
[init.svc.usb_bulk_raw_04]: [running]
[init.svc.usb_bulk_raw_05]: [running]
[init.svc.usb_bulk_raw_06]: [running]
[init.svc.vendor.configstore-hal]: [running]
[init.svc.vold]: [running]
[init.svc.watchdogd]: [running]
[log.tag.stats_log]: [I]
[mount.blackbox]: [1]
[mount.cache]: [1]
[mount.cali]: [1]
[mount.factory_data]: [1]
[net.bt.name]: [Android]
[persist.adb.trace_mask]: []
[persist.dji.log_tag.crash_limit]: [1]
[persist.dji.log_tag.hms_limit]: [1]
[persist.dji.log_tag.manual_limit]: [0]
[persist.gpio-keys.5d_key]: [895]
[persist.gpio-keys.back_key]: [497]
[persist.logd.timestamp]: [monotonic]
[persist.sys.boot.reason]: [reboot,sysmode]
[persist.sys.boot_index]: [84]
[persist.sys.dalvik.vm.lib.2]: [libart.so]
[persist.sys.timezone]: [GMT+1]
[persist.sys.usb.config]: [adb]
[persist.upgrade_reboot]: [0]
[plt.prop.sec_red3_3_state]: [red3_3 state: 3]
[ro.actionable_compatible_property.enabled]: [false]
[ro.allow.mock.location]: [0]
[ro.art.hiddenapi.warning]: [1]
[ro.baseband]: [unknown]
[ro.bionic.ld.warning]: [1]
[ro.board.platform]: [e3t]
[ro.boot.active]: [00010000]
[ro.boot.android_dt_dir]: [/proc/device-tree/firmware_a/android/]
[ro.boot.hardware]: [e3t]
[ro.boot.hw_version]: [04.04.01.00]
[ro.boot.mode]: [normal]
[ro.boot.secure_debug]: [0]
[ro.boot.selinux]: [enforcing]
[ro.boot.slot_suffix]: [1]
[ro.boot.verity]: [1]
[ro.boot.veritymode]: [enforcing]
[ro.bootimage.build.date]: [Mon Oct 19 14:45:31 CST 2026]
[ro.bootimage.build.date.utc]: [1792392331]
[ro.bootimage.build.fingerprint]: [e3t/e3t_zv902/e3t_zv902:9/PD1A.180720.031/38460:userdebug/test-keys]
[ro.bootloader]: [unknown]
[ro.bootmode]: [normal]
[ro.boottime.adbd]: [34601460409]
[ro.boottime.cam_logcat]: [5002863801]
[ro.boottime.console]: [1616476822]
[ro.boottime.coredump_monitor]: [2837354426]
[ro.boottime.dji_agent]: [5311349217]
[ro.boottime.dji_amt]: [5417938020]
[ro.boottime.dji_arhome]: [6127585936]
[ro.boottime.dji_audio]: [4469624218]
[ro.boottime.dji_blackbox_v2]: [4293678124]
[ro.boottime.dji_camera3]: [5099682290]
[ro.boottime.dji_charger]: [5971453124]
[ro.boottime.dji_config_store]: [4271240884]
[ro.boottime.dji_dsp]: [4888325259]
[ro.boottime.dji_ftpd]: [5515055467]
[ro.boottime.dji_gfsk_agent]: [4677089843]
[ro.boottime.dji_glasses]: [4381372395]
[ro.boottime.dji_media_player]: [4344886718]
[ro.boottime.dji_media_server]: [4421095572]
[ro.boottime.dji_network]: [6051387238]
[ro.boottime.dji_sdrs_agent]: [1185377343]
[ro.boottime.dji_sec]: [4753767447]
[ro.boottime.dji_sw_uav]: [6231614842]
[ro.boottime.dji_sys]: [4317698957]
[ro.boottime.dji_upgrade]: [4828706770]
[ro.boottime.dji_wlm]: [4519383072]
[ro.boottime.init]: [382]
[ro.boottime.init.mount_all.early]: [350]
[ro.boottime.init.mount_all.late]: [0]
[ro.boottime.init.selinux]: [228]
[ro.boottime.init_part]: [1186788541]
[ro.boottime.init_second]: [653]
[ro.boottime.interconnect_service]: [6419979165]
[ro.boottime.kernel_log]: [4133178645]
[ro.boottime.load_rcfiles]: [698]
[ro.boottime.logd]: [1066710676]
[ro.boottime.logd-reinit]: [1648913541]
[ro.boottime.netd]: [1663705728]
[ro.boottime.queue_event]: [733]
[ro.boottime.servicemanager]: [1547804687]
[ro.boottime.setup_usb]: [2828786457]
[ro.boottime.start_dji_system]: [1665503124]
[ro.boottime.sys_perf_log]: [12621335673]
[ro.boottime.system_log]: [4173575520]
[ro.boottime.tombstoned]: [2831756249]
[ro.boottime.upgrade_log]: [4198468488]
[ro.boottime.usb_bulk_raw]: [1687257291]
[ro.boottime.usb_bulk_raw_01]: [1692690885]
[ro.boottime.usb_bulk_raw_04]: [1688554426]
[ro.boottime.usb_bulk_raw_05]: [1689961197]
[ro.boottime.usb_bulk_raw_06]: [1691363801]
[ro.boottime.vendor.configstore-hal]: [1661336197]
[ro.boottime.vold]: [1600270572]
[ro.boottime.watchdogd]: [1183068749]
[ro.build.characteristics]: [default]
[ro.build.date]: [Mon Oct 19 14:45:31 CST 2026]
[ro.build.date.utc]: [1792392331]
[ro.build.description]: [e3t_zv902-userdebug 9 PD1A.180720.031 38460 test-keys]
[ro.build.display.id]: [e3t_zv902-userdebug 9 PD1A.180720.031 38460 test-keys]
[ro.build.fingerprint]: [e3t/e3t_zv902/e3t_zv902:9/PD1A.180720.031/38460:userdebug/test-keys]
[ro.build.flavor]: [e3t_zv902-userdebug]
[ro.build.host]: [android-6and9-ci-18.04-slim-img-release-pxrbb]
[ro.build.id]: [PD1A.180720.031]
[ro.build.product]: [e3t_zv902]
[ro.build.tags]: [test-keys]
[ro.build.type]: [userdebug]
[ro.build.user]: []
[ro.build.version.all_codenames]: [REL]
[ro.build.version.base_os]: []
[ro.build.version.codename]: [REL]
[ro.build.version.incremental]: [38460]
[ro.build.version.min_supported_target_sdk]: [17]
[ro.build.version.preview_sdk]: [0]
[ro.build.version.release]: [9]
[ro.build.version.sdk]: [28]
[ro.build.version.security_patch]: [2018-09-05]
[ro.carrier]: [unknown]
[ro.crypto.fuse_sdcard]: [false]
[ro.crypto.state]: [unsupported]
[ro.debuggable]: [1]
[ro.dji.build.version]: [10.00.67.17]
[ro.dji_system_service_ready]: [1]
[ro.expect.recovery_id]: [0x8db34b64a695de94b931d3c396ea758f19b65281000000000000000000000000]
[ro.hardware]: [e3t]
[ro.logd.kernel]: [false]
[ro.logd.size.stats]: [64K]
[ro.persistent_properties.ready]: [true]
[ro.product.board]: [evb2]
[ro.product.brand]: [e3t]
[ro.product.cpu.abi]: [arm64-v8a]
[ro.product.cpu.abilist]: [arm64-v8a]
[ro.product.cpu.abilist32]: []
[ro.product.cpu.abilist64]: [arm64-v8a]
[ro.product.device]: [e3t_zv902]
[ro.product.locale]: [en-US]
[ro.product.manufacturer]: [DJI]
[ro.product.model]: [Android NATIVE on E3T zv902]
[ro.product.name]: [e3t_zv902]
[ro.product.vendor.brand]: [e3t]
[ro.product.vendor.device]: [e3t_zv902]
[ro.product.vendor.manufacturer]: [DJI]
[ro.product.vendor.model]: [Android NATIVE on E3T zv902]
[ro.product.vendor.name]: [e3t_zv902]
[ro.property_service.version]: [2]
[ro.revision]: [0]
[ro.secure]: [1]
[ro.serialno]: []
[ro.treble.enabled]: [true]
[ro.vendor.build.date]: [Mon Oct 19 14:45:31 CST 2026]
[ro.vendor.build.date.utc]: [1792392331]
[ro.vendor.build.fingerprint]: [e3t/e3t_zv902/e3t_zv902:9/PD1A.180720.031/38460:userdebug/test-keys]
[ro.vendor.build.security_patch]: []
[ro.vendor.gnsschip]: [marlin3lite]
[ro.vendor.product.cpu.abilist]: [arm64-v8a]
[ro.vendor.product.cpu.abilist32]: []
[ro.vendor.product.cpu.abilist64]: [arm64-v8a]
[ro.vndk.lite]: [true]
[ro.vndk.version]: [28]
[ro.wifi.channels]: []
[security.perf_harden]: [1]
[selinux.restorecon_recursive]: [/blackbox]
[service.adb.tcp.port]: [5555]
[sys.cpld.fw.ver]: [22.11.11.52]
[sys.monitor.cpu_loading]: [13]
[sys.usb.config]: [acm,adb]
[sys.usb.controller]: [f9100000.usb]
[sys.usb.ffs.ready]: [1]
[sys.usb.state]: [acm,adb]
[tombstoned.max_tombstone_count]: [10]
[usb.init]: [1]
[vold.has_adoptable]: [0]
[vold.has_quota]: [0]
[vold.has_reserved]: [0]
```

## Processes

```shell

/tmp $ ps -AZ
LABEL                          USER           PID  PPID     VSZ    RSS WCHAN            ADDR S NAME                       
u:r:init:s0                    root             1     0   14996   1404 ep_poll             0 S init
u:r:kernel:s0                  root             2     0       0      0 kthreadd            0 S [kthreadd]
u:r:kernel:s0                  root             3     2       0      0 rescuer_thread      0 I [rcu_gp]
u:r:kernel:s0                  root             4     2       0      0 rescuer_thread      0 I [rcu_par_gp]
u:r:kernel:s0                  root             8     2       0      0 rescuer_thread      0 I [mm_percpu_wq]
u:r:kernel:s0                  root             9     2       0      0 0                   0 R [ksoftirqd/0]
u:r:kernel:s0                  root            10     2       0      0 rcu_gp_kthread      0 I [rcu_preempt]
u:r:kernel:s0                  root            11     2       0      0 rcu_boost_kthread   0 S [rcub/0]
u:r:kernel:s0                  root            12     2       0      0 smpboot_thread_fn   0 S [rcuc/0]
u:r:kernel:s0                  root            13     2       0      0 smpboot_thread_fn   0 S [posixcputmr/0]
u:r:kernel:s0                  root            14     2       0      0 smpboot_thread_fn   0 S [migration/0]
u:r:kernel:s0                  root            16     2       0      0 smpboot_thread_fn   0 S [cpuhp/0]
u:r:kernel:s0                  root            17     2       0      0 smpboot_thread_fn   0 S [cpuhp/1]
u:r:kernel:s0                  root            18     2       0      0 smpboot_thread_fn   0 S [migration/1]
u:r:kernel:s0                  root            19     2       0      0 smpboot_thread_fn   0 S [posixcputmr/1]
u:r:kernel:s0                  root            20     2       0      0 smpboot_thread_fn   0 S [rcuc/1]
u:r:kernel:s0                  root            21     2       0      0 smpboot_thread_fn   0 S [ksoftirqd/1]
u:r:kernel:s0                  root            24     2       0      0 smpboot_thread_fn   0 S [cpuhp/2]
u:r:kernel:s0                  root            25     2       0      0 smpboot_thread_fn   0 S [migration/2]
u:r:kernel:s0                  root            26     2       0      0 smpboot_thread_fn   0 S [posixcputmr/2]
u:r:kernel:s0                  root            27     2       0      0 smpboot_thread_fn   0 S [rcuc/2]
u:r:kernel:s0                  root            28     2       0      0 smpboot_thread_fn   0 S [ksoftirqd/2]
u:r:kernel:s0                  root            31     2       0      0 smpboot_thread_fn   0 S [cpuhp/3]
u:r:kernel:s0                  root            32     2       0      0 smpboot_thread_fn   0 S [migration/3]
u:r:kernel:s0                  root            33     2       0      0 smpboot_thread_fn   0 S [posixcputmr/3]
u:r:kernel:s0                  root            34     2       0      0 smpboot_thread_fn   0 S [rcuc/3]
u:r:kernel:s0                  root            35     2       0      0 smpboot_thread_fn   0 S [ksoftirqd/3]
u:r:kernel:s0                  root            38     2       0      0 devtmpfsd           0 S [kdevtmpfs]
u:r:kernel:s0                  root            39     2       0      0 rescuer_thread      0 I [netns]
u:r:kernel:s0                  root            40     2       0      0 rcu_tasks_kthread   0 S [rcu_tasks_kthre]
u:r:kernel:s0                  root            42     2       0      0 kauditd_thread      0 S [kauditd]
u:r:kernel:s0                  root            44     2       0      0 watchdog            0 S [khungtaskd]
u:r:kernel:s0                  root            45     2       0      0 oom_reaper          0 S [oom_reaper]
u:r:kernel:s0                  root            46     2       0      0 rescuer_thread      0 I [writeback]
u:r:kernel:s0                  root            47     2       0      0 kcompactd           0 S [kcompactd0]
u:r:kernel:s0                  root            48     2       0      0 ksm_scan_thread     0 S [ksmd]
u:r:kernel:s0                  root            84     2       0      0 rescuer_thread      0 I [kblockd]
u:r:kernel:s0                  root            85     2       0      0 rescuer_thread      0 I [blkcg_punt_bio]
u:r:kernel:s0                  root            86     2       0      0 irq_thread          0 S [irq/19-dw_axi_d]
u:r:kernel:s0                  root            87     2       0      0 irq_thread          0 S [irq/20-dw_axi_d]
u:r:kernel:s0                  root            88     2       0      0 irq_thread          0 S [irq/21-dw_axi_d]
u:r:kernel:s0                  root            89     2       0      0 irq_thread          0 S [irq/22-dw_axi_d]
u:r:kernel:s0                  root            90     2       0      0 irq_thread          0 S [irq/23-dw_axi_d]
u:r:kernel:s0                  root            91     2       0      0 irq_thread          0 S [irq/24-dw_axi_d]
u:r:kernel:s0                  root            92     2       0      0 irq_thread          0 S [irq/25-dw_axi_d]
u:r:kernel:s0                  root            93     2       0      0 irq_thread          0 S [irq/26-dw_axi_d]
u:r:kernel:s0                  root            95     2       0      0 irq_thread          0 S [irq/28-dw_axi_d]
u:r:kernel:s0                  root            96     2       0      0 irq_thread          0 S [irq/29-dw_axi_d]
u:r:kernel:s0                  root            97     2       0      0 irq_thread          0 S [irq/30-dw_axi_d]
u:r:kernel:s0                  root            98     2       0      0 irq_thread          0 S [irq/31-dw_axi_d]
u:r:kernel:s0                  root            99     2       0      0 irq_thread          0 S [irq/32-dw_axi_d]
u:r:kernel:s0                  root           100     2       0      0 irq_thread          0 S [irq/33-dw_axi_d]
u:r:kernel:s0                  root           101     2       0      0 irq_thread          0 S [irq/34-dw_axi_d]
u:r:kernel:s0                  root           102     2       0      0 irq_thread          0 S [irq/35-dw_axi_d]
u:r:kernel:s0                  root           103     2       0      0 msleep              0 D [kexception]
u:r:kernel:s0                  root           106     2       0      0 irq_thread          0 S [irq/45-f8032000]
u:r:kernel:s0                  root           107     2       0      0 irq_thread          0 S [irq/44-f8031000]
u:r:kernel:s0                  root           108     2       0      0 irq_thread          0 S [irq/46-f8058000]
u:r:kernel:s0                  root           110     2       0      0 worker_thread       0 I [kworker/u8:6-events_unbound]
u:r:kernel:s0                  root           112     2       0      0 irq_thread          0 S [irq/47-f8059000]
u:r:kernel:s0                  root           115     2       0      0 worker_thread       0 I [kworker/u9:0]
u:r:kernel:s0                  root           116     2       0      0 rescuer_thread      0 I [sdhci]
u:r:kernel:s0                  root           117     2       0      0 irq_thread          0 S [irq/15-mmc0]
u:r:kernel:s0                  root           118     2       0      0 irq_thread          0 S [irq/48-f805a000]
u:r:kernel:s0                  root           119     2       0      0 irq_thread          0 S [irq/15-s-mmc0]
u:r:kernel:s0                  root           120     2       0      0 irq_thread          0 S [irq/49-f805b000]
u:r:kernel:s0                  root           121     2       0      0 irq_thread          0 S [irq/67-f5e46000]
u:r:kernel:s0                  root           122     2       0      0 irq_thread          0 S [irq/50-f805e000]
u:r:kernel:s0                  root           125     2       0      0 irq_thread          0 S [irq/103-tca9535]
u:r:kernel:s0                  root           126     2       0      0 irq_thread          0 S [irq/154-lp8758-]
u:r:kernel:s0                  root           127     2       0      0 rescuer_thread      0 I [devfreq_wq]
u:r:kernel:s0                  root           128     2       0      0 kthread_worker_fn   0 S [watchdogd]
u:r:kernel:s0                  root           129     2       0      0 rescuer_thread      0 I [typec_otg_queue]
u:r:kernel:s0                  root           130     2       0      0 ion_heap_deferred_free 0 S [system_heap]
u:r:kernel:s0                  root           131     2       0      0 ion_heap_deferred_free 0 S [carveout_heap1]
u:r:kernel:s0                  root           132     2       0      0 ion_heap_deferred_free 0 S [dspf_heap]
u:r:kernel:s0                  root           133     2       0      0 smpboot_thread_fn   0 S [alarm_irqd/0]
u:r:kernel:s0                  root           134     2       0      0 smpboot_thread_fn   0 S [alarm_irqd/1]
u:r:kernel:s0                  root           135     2       0      0 smpboot_thread_fn   0 S [alarm_irqd/2]
u:r:kernel:s0                  root           136     2       0      0 smpboot_thread_fn   0 S [alarm_irqd/3]
u:r:kernel:s0                  root           137     2       0      0 kswapd              0 S [kswapd0]
u:r:kernel:s0                  root           138     2       0      0 rescuer_thread      0 I [kthrotld]
u:r:kernel:s0                  root           139     2       0      0 irq_thread          0 S [irq/56-f804c000]
u:r:kernel:s0                  root           140     2       0      0 irq_thread          0 S [irq/57-f804c000]
u:r:kernel:s0                  root           141     2       0      0 irq_thread          0 S [irq/58-f804c000]
u:r:kernel:s0                  root           142     2       0      0 irq_thread          0 S [irq/59-f804c000]
u:r:kernel:s0                  root           143     2       0      0 irq_thread          0 S [irq/60-f804c000]
u:r:kernel:s0                  root           144     2       0      0 irq_thread          0 S [irq/61-f804c000]
u:r:kernel:s0                  root           145     2       0      0 irq_thread          0 S [irq/62-f804c000]
u:r:kernel:s0                  root           146     2       0      0 irq_thread          0 S [irq/63-f804c000]
u:r:kernel:s0                  root           147     2       0      0 irq_thread          0 S [irq/75-f34c0000]
u:r:kernel:s0                  root           148     2       0      0 rescuer_thread      0 I [c2d_cmd_workque]
u:r:kernel:s0                  root           149     2       0      0 irq_thread          0 S [irq/84-f5f00000]
u:r:kernel:s0                  root           150     2       0      0 irq_thread          0 S [irq/85-f5f00000]
u:r:kernel:s0                  root           151     2       0      0 irq_thread          0 S [irq/86-f5f00000]
u:r:kernel:s0                  root           152     2       0      0 irq_thread          0 S [irq/87-f5f00000]
u:r:kernel:s0                  root           153     2       0      0 irq_thread          0 S [irq/88-f5f00000]
u:r:kernel:s0                  root           154     2       0      0 smpboot_thread_fn   0 S [isr_threads/0]
u:r:kernel:s0                  root           155     2       0      0 irq_thread          0 S [irq/89-f5f00000]
u:r:kernel:s0                  root           156     2       0      0 smpboot_thread_fn   0 S [isr_threads/1]
u:r:kernel:s0                  root           157     2       0      0 irq_thread          0 S [irq/90-f5f00000]
u:r:kernel:s0                  root           158     2       0      0 smpboot_thread_fn   0 S [isr_threads/2]
u:r:kernel:s0                  root           159     2       0      0 smpboot_thread_fn   0 S [isr_threads/3]
u:r:kernel:s0                  root           160     2       0      0 irq_thread          0 S [irq/83-gimbal_i]
u:r:kernel:s0                  root           161     2       0      0 irq_thread          0 S [irq/154-tps6591]
u:r:kernel:s0                  root           162     2       0      0 irq_thread          0 S [irq/119-apm16d2]
u:r:kernel:s0                  root           163     2       0      0 kthread_worker_fn   0 S [spi1]
u:r:kernel:s0                  root           164     2       0      0 kthread_worker_fn   0 S [spi5]
u:r:kernel:s0                  root           165     2       0      0 irq_thread          0 S [irq/43-spi-dw-s]
u:r:kernel:s0                  root           166     2       0      0 irq_thread          0 S [irq/14-xhci-hcd]
u:r:kernel:s0                  root           167     2       0      0 irq_thread          0 S [irq/71-video-tx]
u:r:kernel:s0                  root           168     2       0      0 irq_thread          0 S [irq/70-f8046000]
u:r:kernel:s0                  root           169     2       0      0 rescuer_thread      0 I [dm_bufio_cache]
u:r:kernel:s0                  root           171     2       0      0 rescuer_thread      0 I [mmc_complete]
u:r:kernel:s0                  root           172     2       0      0 rescuer_thread      0 I [sdhci]
u:r:kernel:s0                  root           173     2       0      0 irq_thread          0 S [irq/17-mmc2]
u:r:kernel:s0                  root           174     2       0      0 rescuer_thread      0 I [sdhci]
u:r:kernel:s0                  root           175     2       0      0 irq_thread          0 S [irq/17-s-mmc2]
u:r:kernel:s0                  root           176     2       0      0 irq_thread          0 S [irq/16-mmc1]
u:r:kernel:s0                  root           178     2       0      0 irq_thread          0 S [irq/16-s-mmc1]
u:r:kernel:s0                  root           179     2       0      0 irq_thread          0 S [irq/98-f9304000]
u:r:kernel:s0                  root           182     2       0      0 irq_thread          0 S [irq/69-ccree]
u:r:kernel:s0                  root           185     2       0      0 irq_thread          0 S [irq/55-timer0_5]
u:r:kernel:s0                  root           244     2       0      0 prb_iter_wait_next  0 S [printk]
u:r:kernel:s0                  root           245     2       0      0 msleep              0 D [printk_saviour]
u:r:kernel:s0                  root           246     2       0      0 async_call_main     0 S [async_call]
u:r:kernel:s0                  root           247     2       0      0 ktop_main           0 R [ktop]
u:r:kernel:s0                  root           248     2       0      0 cpumon_main         0 S [cpumon]
u:r:kernel:s0                  root           249     2       0      0 irq_thread          0 S [irq/5-codec_noc]
u:r:kernel:s0                  root           250     2       0      0 irq_thread          0 S [irq/6-dsp_noc]
u:r:kernel:s0                  root           251     2       0      0 irq_thread          0 S [irq/7-vi_noc]
u:r:kernel:s0                  root           252     2       0      0 irq_thread          0 S [irq/8-vo_noc]
u:r:kernel:s0                  root           253     2       0      0 irq_thread          0 S [irq/9-mctf_noc]
u:r:kernel:s0                  root           254     2       0      0 irq_thread          0 S [irq/10-hsp_noc]
u:r:kernel:s0                  root           255     2       0      0 irq_thread          0 S [irq/11-peri_noc]
u:r:kernel:s0                  root           256     2       0      0 worker_thread       0 I [kworker/0:2-cgroup_pidlist_destroy]
u:r:kernel:s0                  root           257     2       0      0 irq_thread          0 S [irq/12-ms_noc]
u:r:kernel:s0                  root           258     2       0      0 irq_thread          0 S [irq/68-f3304000]
u:r:kernel:s0                  root           259     2       0      0 worker_thread       0 I [kworker/3:3-events]
u:r:kernel:s0                  root           260     2       0      0 irq_thread          0 S [irq/123-back_ke]
u:r:kernel:s0                  root           261     2       0      0 irq_thread          0 S [irq/124-5d_key]
u:r:kernel:s0                  root           263     2       0      0 irq_thread          0 S [irq/227-vop0_bl]
u:r:kernel:s0                  root           264     2       0      0 rt_work_thread      0 S [vop0_rt_work_th]
u:r:kernel:s0                  root           265     2       0      0 dly_work_thread     0 D [vop0_dly_work_t]
u:r:kernel:s0                  root           266     2       0      0 irq_thread          0 S [irq/228-vop1_bl]
u:r:kernel:s0                  root           267     2       0      0 rt_work_thread      0 S [vop1_rt_work_th]
u:r:kernel:s0                  root           268     2       0      0 dly_work_thread     0 S [vop1_dly_work_t]
u:r:kernel:s0                  root           269     2       0      0 irq_thread          0 S [irq/72-dsi_host]
u:r:kernel:s0                  root           270     2       0      0 irq_thread          0 S [irq/73-dsi_host]
u:r:kernel:s0                  root           271     2       0      0 rescuer_thread      0 I [kdmflush]
u:r:kernel:s0                  root           272     2       0      0 rescuer_thread      0 I [kverityd]
u:r:kernel:s0                  root           274     2       0      0 kjournald2          0 S [jbd2/dm-0-8]
u:r:kernel:s0                  root           275     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:kernel:s0                  root           277     2       0      0 kjournald2          0 S [jbd2/mmcblk0p15]
u:r:kernel:s0                  root           278     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:kernel:s0                  root           281     2       0      0 worker_thread       0 I [kworker/3:2H-kblockd]
u:r:kernel:s0                  root           282     2       0      0 irq_thread          0 S [irq/74-rfe_pipe]
u:r:kernel:s0                  root           283     2       0      0 irq_thread          0 S [irq/229-raw_pip]
u:r:kernel:s0                  root           284     2       0      0 irq_thread          0 S [irq/230-yuv_pip]
u:r:kernel:s0                  root           285     2       0      0 kthread_worker_fn   0 S [pygen_proc_task]
u:r:kernel:s0                  root           286     2       0      0 irq_thread          0 S [irq/231-pyramid]
u:r:kernel:s0                  root           287     2       0      0 kthread_worker_fn   0 S [pyrecon_proc_ta]
u:r:kernel:s0                  root           288     2       0      0 irq_thread          0 S [irq/232-pyrd_re]
u:r:kernel:s0                  root           289     2       0      0 kthread_worker_fn   0 S [ycc_proc_task]
u:r:kernel:s0                  root           294     2       0      0 irq_thread          0 S [irq/233-gdc2_ea]
u:r:kernel:s0                  root           295     2       0      0 kthread_worker_fn   0 S [gdc_proc_task]
u:r:kernel:s0                  root           296     2       0      0 irq_thread          0 S [irq/234-mctf2_e]
u:r:kernel:s0                  root           297     2       0      0 kthread_worker_fn   0 S [mctf_worker]
u:r:kernel:s0                  root           298     2       0      0 irq_thread          0 S [irq/235-vtx]
u:r:kernel:s0                  root           299     2       0      0 kthread_worker_fn   0 S [vtx_wrap_worker]
u:r:kernel:s0                  root           300     2       0      0 irq_thread          0 S [irq/236-vbus_co]
u:r:kernel:s0                  root           301     2       0      0 irq_thread          0 S [irq/237-idi2vbu]
u:r:kernel:s0                  root           302     2       0      0 irq_thread          0 S [irq/238-idi2vbu]
u:r:kernel:s0                  root           303     2       0      0 irq_thread          0 S [irq/239-idi2vbu]
u:r:kernel:s0                  root           304     2       0      0 irq_thread          0 S [irq/240-csi_hos]
u:r:kernel:s0                  root           305     2       0      0 irq_thread          0 S [irq/241-csi_hos]
u:r:kernel:s0                  root           306     2       0      0 irq_thread          0 S [irq/242-csi_hos]
u:r:kernel:s0                  root           307     2       0      0 irq_thread          0 S [irq/243-mipi]
u:r:kernel:s0                  root           308     2       0      0 irq_thread          0 S [irq/244-combo0]
u:r:kernel:s0                  root           309     2       0      0 irq_thread          0 S [irq/245-combo1]
u:r:kernel:s0                  root           310     2       0      0 irq_thread          0 S [irq/246-senc_sh]
u:r:kernel:s0                  root           311     2       0      0 irq_thread          0 S [irq/247-senc_se]
u:r:kernel:s0                  root           314     2       0      0 rescuer_thread      0 I [mmc_complete]
u:r:vendor_init:s0             root           315     1    6532    392 do_sys_poll         0 S init
u:r:vendor_init:s0             root           316     1    6532    392 do_sys_poll         0 S init
u:r:kernel:s0                  root           317     2       0      0 kjournald2          0 S [jbd2/mmcblk0p22]
u:r:kernel:s0                  root           318     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:kernel:s0                  root           322     2       0      0 kjournald2          0 S [jbd2/mmcblk0p21]
u:r:kernel:s0                  root           323     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:logd:s0                    logd           325     1   26256   4612 sigsuspend          0 S logd
u:r:kernel:s0                  root           327     2       0      0 kjournald2          0 S [jbd2/mmcblk0p5-]
u:r:kernel:s0                  root           328     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:watchdogd:s0               root           335     1    6532    252 hrtimer_nanosleep   0 S watchdogd
u:r:dji_services:s0            root           336     1   90548   9100 hrtimer_nanosleep   0 S dji_sdrs_agent
u:r:kernel:s0                  root           339     2       0      0 kjournald2          0 S [jbd2/mmcblk0p20]
u:r:kernel:s0                  root           340     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:kernel:s0                  root           344     2       0      0 kjournald2          0 S [jbd2/mmcblk0p4-]
u:r:kernel:s0                  root           345     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:kernel:s0                  root           349     2       0      0 kjournald2          0 S [jbd2/mmcblk0p23]
u:r:kernel:s0                  root           350     2       0      0 rescuer_thread      0 I [ext4-rsv-conver]
u:r:servicemanager:s0          system         351     1   11640   3276 binder_thread_read  0 S servicemanager
u:r:vold:s0                    root           352     1   41436   4004 binder_thread_read  0 S vold
u:r:shell:s0                   root           353     1    9384   2636 wait_woken          0 S sh
u:r:dji_services:s0            system         356     1   13360   2752 futex_wait_queue_me 0 S android.hardware.configstore@1.1-service
u:r:netd:s0                    root           357     1 2144104   2872 futex_wait_queue_me 0 S netd
u:r:dji_services:s0            root           359     1    9284   2472 hrtimer_nanosleep   0 S usb_bulk_raw
u:r:dji_services:s0            root           360     1    9284   2304 hrtimer_nanosleep   0 S usb_bulk_raw
u:r:dji_services:s0            root           361     1    9284   2496 hrtimer_nanosleep   0 S usb_bulk_raw
u:r:dji_services:s0            root           362     1    9284   2472 hrtimer_nanosleep   0 S usb_bulk_raw
u:r:dji_services:s0            root           363     1    9284   2472 hrtimer_nanosleep   0 S usb_bulk_raw
u:r:kernel:s0                  root           482     2       0      0 irq_thread          0 S [irq/66-ttyS1]
u:r:kernel:s0                  root           723     2       0      0 irq_thread          0 S [irq/40-ddr_trac]
u:r:kernel:s0                  root           724     2       0      0 vfs_process_work    0 S [ddr_trace_0]
u:r:kernel:s0                  root           727     2       0      0 irq_thread          0 S [irq/39-f5f0c000]
u:r:kernel:s0                  root           742     2       0      0 irq_thread          0 S [irq/252-dsp_cor]
u:r:kernel:s0                  root           743     2       0      0 irq_thread          0 S [irq/254-dsp_cor]
u:r:kernel:s0                  root           744     2       0      0 irq_thread          0 S [irq/255-dsp_cor]
u:r:kernel:s0                  root           745     2       0      0 irq_thread          0 S [irq/256-dsp_cor]
u:r:kernel:s0                  root           746     2       0      0 irq_thread          0 S [irq/257-dsp_cor]
u:r:kernel:s0                  root           747     2       0      0 irq_thread          0 S [irq/258-dsp_cor]
u:r:kernel:s0                  root           748     2       0      0 irq_thread          0 S [irq/259-dsp_cor]
u:r:kernel:s0                  root           749     2       0      0 irq_thread          0 S [irq/260-dsp_cor]
u:r:kernel:s0                  root           750     2       0      0 irq_thread          0 S [irq/262-dsp_cor]
u:r:kernel:s0                  root           751     2       0      0 irq_thread          0 S [irq/263-dsp_cor]
u:r:kernel:s0                  root           752     2       0      0 irq_thread          0 S [irq/264-dsp_cor]
u:r:kernel:s0                  root           753     2       0      0 irq_thread          0 S [irq/265-dsp_cor]
u:r:kernel:s0                  root           754     2       0      0 irq_thread          0 S [irq/266-dsp_cor]
u:r:kernel:s0                  root           755     2       0      0 irq_thread          0 S [irq/267-dsp_cor]
u:r:kernel:s0                  root           772     2       0      0 irq_thread          0 S [irq/79-codec-h2]
u:r:kernel:s0                  root           773     2       0      0 irq_thread          0 S [irq/80-codec-h2]
u:r:kernel:s0                  root           775     2       0      0 kthread_worker_fn   0 S [hevc_hl_proc_ta]
u:r:tombstoned:s0              tombstoned     782     1    9484   2224 ep_poll             0 S tombstoned
u:r:kernel:s0                  root           783     2       0      0 irq_thread          0 S [irq/82-cmd_dma]
u:r:dji_services:s0            root           789     1    9896   2584 hrtimer_nanosleep   0 S coredump_monitor
u:r:kernel:s0                  root           792     2       0      0 irq_thread          0 S [irq/81-r2t]
u:r:kernel:s0                  root           809     2       0      0 worker_thread       0 I [kworker/0:3-pm]
u:r:kernel:s0                  root           816     2       0      0 irq_thread          0 S [irq/78-h264enc]
u:r:kernel:s0                  root           817     2       0      0 kthread_worker_fn   0 S [h264_hl_proc_ta]
u:r:kernel:s0                  root           831     2       0      0 irq_thread          0 S [irq/76-hjpgenc_]
u:r:kernel:s0                  root           844     2       0      0 irq_thread          0 S [irq/77-hantrode]
u:r:kernel:s0                  root           885     2       0      0 sleep_thread        0 S [file-storage]
u:r:kernel:s0                  root           899     2       0      0 rescuer_thread      0 I [cfg80211]
u:r:dji_services:s0            root           952     1   11376   2992 prb_iter_wait_next  0 S dji_kmsg
u:r:start_dji_system:s0        root           968     1   15524   2532 optee_supp_recv     0 S tee-supplicant
u:r:dji_services:s0            root           971     1  118904   4116 hrtimer_nanosleep   0 S dji_config_store
u:r:dji_services:s0            root           973     1  113152  13424 hrtimer_nanosleep   0 S dji_blackbox
u:r:dji_services:s0            root           976     1  311844  13988 hrtimer_nanosleep   0 S dji_sys
u:r:dji_services:s0            root           978     1   64528   4132 binder_thread_read  0 S dji_media_player
u:r:dji_services:s0            root           982     1  373012   6616 hrtimer_nanosleep   0 S dji_media_server
u:r:dji_audio:s0               root           984     1   52320   3732 binder_thread_read  0 S dji_audio
u:r:dji_wlm:s0                 root           986     1   80224   8056 hrtimer_nanosleep   0 S dji_wlm
u:r:dji_gfsk_agent:s0          root          1008     1   71320   5764 hrtimer_nanosleep   0 S dji_gfsk_agent
u:r:dji_sec:s0                 root          1014     1   74888   5268 hrtimer_nanosleep   0 S dji_sec
u:r:dji_upgrade:s0             root          1027     1  118032   6012 hrtimer_nanosleep   0 S dji_upgrade
u:r:dji_services:s0            root          1031     1   91952   5540 hrtimer_nanosleep   0 S ss_dsp_manager
u:r:dji_services:s0            root          1045     1    9384   2656 sigsuspend          0 S sh
u:r:start_dji_camera:s0        root          1054     1    9384   2576 sigsuspend          0 S sh
u:r:dji_services:s0            root          1063  1045   35200   3884 do_sys_poll         0 S logcat
u:r:start_dji_camera:s0        root          1064  1054  508220  13356 futex_wait_queue_me 0 S dji_camera3
u:r:dji_services:s0            root          1079     1   58896   3588 hrtimer_nanosleep   0 S dji_agent
u:r:dji_amt:s0                 root          1116     1   58308   4660 hrtimer_nanosleep   0 S dji_amt
u:r:dji_services:s0            root          1129     1   11152   2788 do_sys_poll         0 S dji_ftpd
u:r:dji_services:s0            root          1215     1   64092   5544 hrtimer_nanosleep   0 S dji_charger
u:r:dji_services:s0            root          1241     1   69104   5508 hrtimer_nanosleep   0 S dji_network
u:r:dji_services:s0            root          1273     1  159816   4128 hrtimer_nanosleep   0 S dji_arhome
u:r:dji_services:s0            root          1290     1   78452   4016 __skb_wait_for_more_packets 0 S dji_sw_uav
u:r:interconnect_service:s0    root          1329     1   63080   3812 hrtimer_nanosleep   0 S interconnect_service
u:r:kernel:s0                  root          1422     2       0      0 msleep              0 D [mfi_dev_timer_k]
u:r:dji_services:s0            root          1853     1   16620   3168 hrtimer_nanosleep   0 S bt_light_bluez_app
u:r:kernel:s0                  root          2053     2       0      0 frog_processor      0 S [fr_processor[0]]
u:r:kernel:s0                  root          2054     2       0      0 frog_processor      0 S [fr_processor[1]]
u:r:dji_services:s0            root          2216     1   13124   2900 hrtimer_nanosleep   0 S sys_perf_monitor
u:r:kernel:s0                  root          2225     2       0      0 sdiohal_tx_down     0 S [sdiohal_tx_thre]
u:r:kernel:s0                  root          2226     2       0      0 sdiohal_rx_down     0 S [sdiohal_rx_thre]
u:r:kernel:s0                  root          2231     2       0      0 rescuer_thread      0 I [WCN_LOOPCHECK_Q]
u:r:start_dji_system:s0        root          2361     1    9740   2896 inet_csk_accept     0 S irisdbgd
u:r:kernel:s0                  root          2380     2       0      0 irq_thread          0 S [irq/65-ttyS4]
u:r:su:s0                      root          2430     1   20452   2016 0                   0 S adbd
u:r:kernel:s0                  root          2449     2       0      0 rescuer_thread      0 I [SPRDBT_RX_QUEUE]
u:r:dji_services:s0            root          2455     1   14420   2764 hrtimer_nanosleep   0 S dji_engpc
u:r:su:s0                      root          2882     1    9384   3216 sigsuspend          0 S sh
u:r:su:s0                      root          3258  2882  463892  35784 hrtimer_nanosleep   0 S dji_glasses
u:r:logpersist:s0              root          3417     1   10204   3172 __skb_wait_for_more_packets 0 S logcat
u:r:logpersist:s0              root          3418     1   10204   3180 __skb_wait_for_more_packets 0 S logcat
u:r:kernel:s0                  root          3883     2       0      0 worker_thread       0 I [kworker/1:0-pm]
u:r:su:s0                      root          4052     1    2276   1056 do_select           0 S busybox
u:r:kernel:s0                  root          6015     2       0      0 worker_thread       0 I [kworker/2:2-events]
u:r:kernel:s0                  root          8725     2       0      0 worker_thread       0 I [kworker/u8:1-events_unbound]
u:r:kernel:s0                  root          8825     2       0      0 worker_thread       0 I [kworker/u8:2-events_unbound]
u:r:kernel:s0                  root         10567     2       0      0 worker_thread       0 I [kworker/3:0-pm]
u:r:kernel:s0                  root         11001     2       0      0 worker_thread       0 I [kworker/2:0-pm]
u:r:kernel:s0                  root         11365     2       0      0 worker_thread       0 I [kworker/0:0H-mmc_complete]
u:r:kernel:s0                  root         12314     2       0      0 worker_thread       0 I [kworker/1:1H-kblockd]
u:r:kernel:s0                  root         12351     2       0      0 worker_thread       0 I [kworker/2:1H-kblockd]
u:r:kernel:s0                  root         12355     2       0      0 worker_thread       0 I [kworker/u8:4-events_unbound]
u:r:kernel:s0                  root         12356     2       0      0 worker_thread       0 I [kworker/1:1]
u:r:kernel:s0                  root         12368     2       0      0 irq_thread          0 S [irq/13-dwc3]
u:r:kernel:s0                  root         12531     2       0      0 worker_thread       0 I [kworker/0:2H-mmc_complete]
u:r:kernel:s0                  root         12797     2       0      0 worker_thread       0 I [kworker/3:1H-kblockd]
u:r:kernel:s0                  root         12830     2       0      0 worker_thread       0 I [kworker/1:0H-kblockd]
u:r:kernel:s0                  root         12854     2       0      0 worker_thread       0 I [kworker/2:0H-kblockd]
u:r:kernel:s0                  root         12928     2       0      0 worker_thread       0 I [kworker/u8:0]
u:r:kernel:s0                  root         13020     2       0      0 worker_thread       0 I [kworker/0:1H-mmc_complete]
u:r:kernel:s0                  root         13309     2       0      0 worker_thread       0 I [kworker/3:0H]
u:r:kernel:s0                  root         13360     2       0      0 worker_thread       0 I [kworker/1:2H]
u:r:su:s0                      root         13373  2430    9384   3180 sigsuspend          0 S sh
u:r:su:s0                      root         13375 13373   11848   3664 0                   0 R ps
```

## Devices
### dji_glasses launch state

```shell
$ getprop init.svc.dji_glasses
stopped
$ readlink /proc/3258/exe
/system/bin/dji_glasses
$ tr '\000' ' ' < /proc/3258/cmdline
/system/bin/dji_glasses -g -f
$ cat /proc/3258/attr/current
u:r:su:s0
```

At this snapshot PID 3258 was running even though init reported the service as stopped. Its parent in the process listing is the interactive `sh` (PID 2882), and its SELinux context is `u:r:su:s0`, consistent with a shell-launched process rather than the init-managed service. The pasted `__PWD:/tmp` suffix appears to be shell-prompt text joined to these `/proc` reads, which do not emit a trailing newline; verify argument boundaries with `tr '\000' '\n'` if they matter.

## Devices

```shell

/tmp $ ls -l /dev
total 8
drwx--x--x 2 root   root       1400 1970-01-01 01:00 __properties__
crw------- 1 root   root    10, 112 1970-01-01 01:00 aoa
crw------- 1 root   root   120,   0 1970-01-01 01:00 apm16d24-dev
crw------- 1 root   root    10, 113 1970-01-01 01:00 apple_roleswitch
crw------- 1 root   root    10, 114 1970-01-01 01:00 applecp
crw-rw-rw- 1 root   root    10, 109 1970-01-01 01:00 ashmem
crw-rw-rw- 1 root   root    10, 104 1970-01-01 01:00 binder
drwxr-xr-x 6 root   root        740 1970-01-01 01:00 block
drwxr-xr-x 3 root   root         60 1970-01-01 01:00 bus
crw------- 1 root   root    10,  82 1970-01-01 01:00 cam_data_intf
crw------- 1 root   root    10,  79 1970-01-01 01:00 cam_ecg
crw------- 1 root   root    10,  85 1970-01-01 01:00 cam_uio
lrwxrwxrwx 1 root   root         52 1970-01-01 01:00 camfreq -> /sys/devices/platform/soc/soc:cam_clk_domain/camfreq
drw------- 2 root   root          0 1970-01-01 01:00 cg2_bpf
drwx--x--x 2 system system       60 1970-01-01 01:00 cgroup_info
crw------- 1 root   root    10,  71 1970-01-01 01:00 codec-cdma
crw------- 1 root   root    10,  69 1970-01-01 01:00 codec-h264e
crw------- 1 root   root    10,  72 1970-01-01 01:00 codec-h265e
crw------- 1 root   root    10,  70 1970-01-01 01:00 codec-r2t
crw------- 1 root   root     5,   1 1970-01-01 01:00 console
crw------- 1 root   root    10, 101 1970-01-01 01:00 cpu_dma_latency
drwxr-xr-x 4 system system        0 2026-11-23 00:51 cpuctl
dr-xr-xr-x 7 system system        0 1970-01-01 01:00 cpuset
crw------- 1 root   root    10, 236 1970-01-01 01:00 device-mapper
crw------- 1 root   root    10,  98 1970-01-01 01:00 dji_ae
crw------- 1 root   root    10,  97 1970-01-01 01:00 dji_af
crw------- 1 root   root    10,  96 1970-01-01 01:00 dji_awb
crw------- 1 root   root    10,  94 1970-01-01 01:00 dji_blc
crw------- 1 root   root    10, 123 1970-01-01 01:00 dji_c2d
crw------- 1 root   root    10,  86 1970-01-01 01:00 dji_disp_consumer
crw------- 1 root   root    10, 124 1970-01-01 01:00 dji_display
crw------- 1 root   root    10,  90 1970-01-01 01:00 dji_frog
crw------- 1 root   root    10,  84 1970-01-01 01:00 dji_gdc_v2
crw------- 1 root   root    10,  81 1970-01-01 01:00 dji_hardlink_drv
crw------- 1 root   root    10, 125 1970-01-01 01:00 dji_kevent
crw------- 1 root   root    10, 126 1970-01-01 01:00 dji_khms
crw------- 1 root   root    10,  83 1970-01-01 01:00 dji_mctf_v2
crw------- 1 root   root    10,  93 1970-01-01 01:00 dji_pfr
crw------- 1 root   root    10,  89 1970-01-01 01:00 dji_pyrd_gen_v2
crw------- 1 root   root    10,  88 1970-01-01 01:00 dji_pyrd_recon
crw------- 1 root   root    10,  92 1970-01-01 01:00 dji_rgb_stat
crw------- 1 root   root    10,  95 1970-01-01 01:00 dji_rltm
crw------- 1 root   root    10, 122 1970-01-01 01:00 dji_video_in
crw------- 1 root   root    10,   0 1970-01-01 01:00 dji_vtx
crw------- 1 root   root    10,  87 1970-01-01 01:00 dji_ycc
crw------- 1 root   root    10,  75 1970-01-01 01:00 dsp_core0
crw------- 1 root   root    10,  74 1970-01-01 01:00 dsp_core1
crw------- 1 root   root    10,  77 1970-01-01 01:00 eagle_dsp0_pm
crw------- 1 root   root    10,  76 1970-01-01 01:00 eagle_dsp1_pm
crw------- 1 root   root    10,  78 1970-01-01 01:00 eagle_vir_dsp_pm
-rw-r--r-- 1 logd   logd         24 1970-01-01 01:00 event-log-tags
lrwxrwxrwx 1 root   root         13 1970-01-01 01:00 fd -> /proc/self/fd
crw------- 1 root   root    10, 100 1970-01-01 01:00 fg_pm
crw------- 1 root   root   241,   1 1970-01-01 01:00 fmem
drwxrwx--- 2 root   system       60 1970-01-01 01:00 fscklogs
crw-rw-rw- 1 root   root     1,   7 1970-01-01 01:00 full
crw------- 1 root   root    10,  65 2026-11-23 00:51 gnss_common_ctl
crw------- 1 root   root    10,  64 2026-11-23 00:51 gnss_pmnotify_ctl
crw------- 1 root   root   254,   0 1970-01-01 01:00 gpiochip0
crw------- 1 root   root   254,   1 1970-01-01 01:00 gpiochip1
crw------- 1 root   root   254,   2 1970-01-01 01:00 gpiochip2
crw------- 1 root   root   254,   3 1970-01-01 01:00 gpiochip3
crw------- 1 root   root   254,   4 1970-01-01 01:00 gpiochip4
crw------- 1 root   root    10,  67 1970-01-01 01:00 hantrodec
crw-rw-rw- 1 root   root    10, 103 1970-01-01 01:00 hwbinder
crw------- 1 root   root    10,  68 1970-01-01 01:00 hx280
crw------- 1 root   root    89,   0 1970-01-01 01:00 i2c-0
crw------- 1 root   root    89,   1 1970-01-01 01:00 i2c-1
crw------- 1 root   root    89,  12 1970-01-01 01:00 i2c-12
crw------- 1 root   root    89,  13 1970-01-01 01:00 i2c-13
crw------- 1 root   root    89,  15 1970-01-01 01:00 i2c-15
crw------- 1 root   root    89,   6 1970-01-01 01:00 i2c-6
crw------- 1 root   root    89,   7 1970-01-01 01:00 i2c-7
crw------- 1 root   root    89,   8 1970-01-01 01:00 i2c-8
crw------- 1 root   root    89,   9 1970-01-01 01:00 i2c-9
crw------- 1 root   root    10,  80 1970-01-01 01:00 icc_dev
drwxr-xr-x 2 root   root        120 2026-11-23 00:52 input
crw-rw-rw- 1 root   root    10, 127 1970-01-01 01:00 ion
crw------- 1 root   root    10,  99 1970-01-01 01:00 iris
crw-r--r-- 1 root   root     1,  11 1970-01-01 01:00 kmsg
crw--w--w- 1 root   root     1,  11 1970-01-01 01:00 kmsg_debug
crw------- 1 root   root    10, 111 1970-01-01 01:00 linesync
crw------- 1 root   root    10, 107 1970-01-01 01:00 log_events
crw------- 1 root   root    10, 108 1970-01-01 01:00 log_main
crw------- 1 root   root    10, 106 1970-01-01 01:00 log_radio
crw------- 1 root   root    10, 105 1970-01-01 01:00 log_system
drwxr-xr-x 2 root   root         60 1970-01-01 01:00 mapper
crw------- 1 root   root   157,   0 1970-01-01 01:00 master_nrf52810_rx
crw------- 1 root   root   252,   0 1970-01-01 01:00 media0
crw------- 1 root   root     1,   1 1970-01-01 01:00 mem
drwx------ 4 root   system        0 1970-01-01 01:00 memcg
crw------- 1 root   root   243,   0 1970-01-01 01:00 mmcblk0rpmb
crw------- 1 root   root    10,  91 1970-01-01 01:00 mrt_dev
crw------- 1 root   root    10, 116 1970-01-01 01:00 noc_qos
crw-rw-rw- 1 root   root     1,   3 1970-01-01 01:00 null
crw------- 1 root   root    10, 120 1970-01-01 01:00 oct
crw------- 1 root   root    10, 110 1970-01-01 01:00 perf_trace
crw------- 1 root   root    10, 119 1970-01-01 01:00 peri-timestamp
crw------- 1 root   root    10, 118 1970-01-01 01:00 pps-dji-gpio
crw-rw-rw- 1 root   root     5,   2 2026-11-23 02:20 ptmx
drwxr-xr-x 2 root   root          0 1970-01-01 01:00 pts
crw-rw-rw- 1 root   root     1,   8 1970-01-01 01:00 random
crw------- 1 root   root    10, 242 1970-01-01 01:00 rfkill
crw------- 1 root   root    10, 115 1970-01-01 01:00 shared_mem
crw------- 1 root   root   156,   0 1970-01-01 01:00 slave_nrf52810_rx
crw------- 1 root   root   255,   0 2026-11-23 00:51 slog_wcn0
crw------- 1 root   root   255,   1 2026-11-23 00:51 slog_wcn1
drwxr-xr-x 2 root   root        120 1970-01-01 01:00 snd
drwxr-xr-x 2 root   root        280 1970-01-01 01:00 socket
crw------- 1 root   root   153,   0 1970-01-01 01:00 spidev5.0
crwsrwsrwt 1 root   root   246,   0 1970-01-01 01:00 sysak
crw------- 1 root   root   248,   0 1970-01-01 01:00 tee0
crw------- 1 root   root   248,  16 1970-01-01 01:00 teepriv0
crw-rw-rw- 1 root   root     5,   0 1970-01-01 01:00 tty
crw------- 1 root   root     4,   0 1970-01-01 01:00 tty0
crw------- 1 root   root     4,   1 1970-01-01 01:00 tty1
crw------- 1 root   root     4,  10 1970-01-01 01:00 tty10
crw------- 1 root   root     4,  11 1970-01-01 01:00 tty11
crw------- 1 root   root     4,  12 1970-01-01 01:00 tty12
crw------- 1 root   root     4,  13 1970-01-01 01:00 tty13
crw------- 1 root   root     4,  14 1970-01-01 01:00 tty14
crw------- 1 root   root     4,  15 1970-01-01 01:00 tty15
crw------- 1 root   root     4,  16 1970-01-01 01:00 tty16
crw------- 1 root   root     4,  17 1970-01-01 01:00 tty17
crw------- 1 root   root     4,  18 1970-01-01 01:00 tty18
crw------- 1 root   root     4,  19 1970-01-01 01:00 tty19
crw------- 1 root   root     4,   2 1970-01-01 01:00 tty2
crw------- 1 root   root     4,  20 1970-01-01 01:00 tty20
crw------- 1 root   root     4,  21 1970-01-01 01:00 tty21
crw------- 1 root   root     4,  22 1970-01-01 01:00 tty22
crw------- 1 root   root     4,  23 1970-01-01 01:00 tty23
crw------- 1 root   root     4,  24 1970-01-01 01:00 tty24
crw------- 1 root   root     4,  25 1970-01-01 01:00 tty25
crw------- 1 root   root     4,  26 1970-01-01 01:00 tty26
crw------- 1 root   root     4,  27 1970-01-01 01:00 tty27
crw------- 1 root   root     4,  28 1970-01-01 01:00 tty28
crw------- 1 root   root     4,  29 1970-01-01 01:00 tty29
crw------- 1 root   root     4,   3 1970-01-01 01:00 tty3
crw------- 1 root   root     4,  30 1970-01-01 01:00 tty30
crw------- 1 root   root     4,  31 1970-01-01 01:00 tty31
crw------- 1 root   root     4,  32 1970-01-01 01:00 tty32
crw------- 1 root   root     4,  33 1970-01-01 01:00 tty33
crw------- 1 root   root     4,  34 1970-01-01 01:00 tty34
crw------- 1 root   root     4,  35 1970-01-01 01:00 tty35
crw------- 1 root   root     4,  36 1970-01-01 01:00 tty36
crw------- 1 root   root     4,  37 1970-01-01 01:00 tty37
crw------- 1 root   root     4,  38 1970-01-01 01:00 tty38
crw------- 1 root   root     4,  39 1970-01-01 01:00 tty39
crw------- 1 root   root     4,   4 1970-01-01 01:00 tty4
crw------- 1 root   root     4,  40 1970-01-01 01:00 tty40
crw------- 1 root   root     4,  41 1970-01-01 01:00 tty41
crw------- 1 root   root     4,  42 1970-01-01 01:00 tty42
crw------- 1 root   root     4,  43 1970-01-01 01:00 tty43
crw------- 1 root   root     4,  44 1970-01-01 01:00 tty44
crw------- 1 root   root     4,  45 1970-01-01 01:00 tty45
crw------- 1 root   root     4,  46 1970-01-01 01:00 tty46
crw------- 1 root   root     4,  47 1970-01-01 01:00 tty47
crw------- 1 root   root     4,  48 1970-01-01 01:00 tty48
crw------- 1 root   root     4,  49 1970-01-01 01:00 tty49
crw------- 1 root   root     4,   5 1970-01-01 01:00 tty5
crw------- 1 root   root     4,  50 1970-01-01 01:00 tty50
crw------- 1 root   root     4,  51 1970-01-01 01:00 tty51
crw------- 1 root   root     4,  52 1970-01-01 01:00 tty52
crw------- 1 root   root     4,  53 1970-01-01 01:00 tty53
crw------- 1 root   root     4,  54 1970-01-01 01:00 tty54
crw------- 1 root   root     4,  55 1970-01-01 01:00 tty55
crw------- 1 root   root     4,  56 1970-01-01 01:00 tty56
crw------- 1 root   root     4,  57 1970-01-01 01:00 tty57
crw------- 1 root   root     4,  58 1970-01-01 01:00 tty58
crw------- 1 root   root     4,  59 1970-01-01 01:00 tty59
crw------- 1 root   root     4,   6 1970-01-01 01:00 tty6
crw------- 1 root   root     4,  60 1970-01-01 01:00 tty60
crw------- 1 root   root     4,  61 1970-01-01 01:00 tty61
crw------- 1 root   root     4,  62 1970-01-01 01:00 tty62
crw------- 1 root   root     4,  63 1970-01-01 01:00 tty63
crw------- 1 root   root     4,   7 1970-01-01 01:00 tty7
crw------- 1 root   root     4,   8 1970-01-01 01:00 tty8
crw------- 1 root   root     4,   9 1970-01-01 01:00 tty9
crw------- 1 root   root   240,   0 2026-11-23 00:51 ttyBT0
crw------- 1 root   root   244,   0 1970-01-01 01:00 ttyGS0
crw------- 1 root   root     4,  64 1970-01-01 01:00 ttyS0
crw------- 1 root   root     4,  72 1970-01-01 01:00 ttyS0_raw
crw------- 1 root   root     4,  65 1970-01-01 01:00 ttyS1
crw------- 1 root   root     4,  73 1970-01-01 01:00 ttyS1_raw
crw------- 1 root   root     4,  66 1970-01-01 01:00 ttyS2
crw------- 1 root   root     4,  67 1970-01-01 01:00 ttyS3
crw------- 1 root   root     4,  68 2026-11-23 02:30 ttyS4
crw------- 1 root   root     4,  76 1970-01-01 01:00 ttyS4_raw
crw------- 1 root   root     4,  69 1970-01-01 01:00 ttyS5
crw------- 1 root   root     4,  70 1970-01-01 01:00 ttyS6
crw------- 1 root   root     4,  71 1970-01-01 01:00 ttyS7
crw------- 1 root   root     4, 128 1970-01-01 01:00 ttySec0
crw------- 1 root   root    10, 117 1970-01-01 01:00 tzasc
crw------- 1 root   root   242,   0 1970-01-01 01:00 tzlog
crw------- 1 root   root    10, 223 1970-01-01 01:00 uinput
crw------- 1 root   root    10, 121 1970-01-01 01:00 unrd
crw-rw-rw- 1 root   root     1,   9 1970-01-01 01:00 urandom
drwxr-xr-x 8 root   root        160 1970-01-01 01:00 usb-ffs
crw------- 1 root   root    81,  20 1970-01-01 01:00 v4l-subdev0
crw------- 1 root   root    81,  21 1970-01-01 01:00 v4l-subdev1
crw------- 1 root   root    81,  30 1970-01-01 01:00 v4l-subdev10
crw------- 1 root   root    81,  31 1970-01-01 01:00 v4l-subdev11
crw------- 1 root   root    81,  32 1970-01-01 01:00 v4l-subdev12
crw------- 1 root   root    81,  33 1970-01-01 01:00 v4l-subdev13
crw------- 1 root   root    81,  34 1970-01-01 01:00 v4l-subdev14
crw------- 1 root   root    81,  35 1970-01-01 01:00 v4l-subdev15
crw------- 1 root   root    81,  36 1970-01-01 01:00 v4l-subdev16
crw------- 1 root   root    81,  37 1970-01-01 01:00 v4l-subdev17
crw------- 1 root   root    81,  38 1970-01-01 01:00 v4l-subdev18
crw------- 1 root   root    81,  39 1970-01-01 01:00 v4l-subdev19
crw------- 1 root   root    81,  22 1970-01-01 01:00 v4l-subdev2
crw------- 1 root   root    81,  40 1970-01-01 01:00 v4l-subdev20
crw------- 1 root   root    81,  41 1970-01-01 01:00 v4l-subdev21
crw------- 1 root   root    81,  42 1970-01-01 01:00 v4l-subdev22
crw------- 1 root   root    81,  43 1970-01-01 01:00 v4l-subdev23
crw------- 1 root   root    81,  23 1970-01-01 01:00 v4l-subdev3
crw------- 1 root   root    81,  24 1970-01-01 01:00 v4l-subdev4
crw------- 1 root   root    81,  25 1970-01-01 01:00 v4l-subdev5
crw------- 1 root   root    81,  26 1970-01-01 01:00 v4l-subdev6
crw------- 1 root   root    81,  27 1970-01-01 01:00 v4l-subdev7
crw------- 1 root   root    81,  28 1970-01-01 01:00 v4l-subdev8
crw------- 1 root   root    81,  29 1970-01-01 01:00 v4l-subdev9
crw------- 1 root   root    10,  73 1970-01-01 01:00 vc8000ej_pm
crw------- 1 root   root     7,   0 1970-01-01 01:00 vcs
crw------- 1 root   root     7,   1 1970-01-01 01:00 vcs1
crw------- 1 root   root     7, 128 1970-01-01 01:00 vcsa
crw------- 1 root   root     7, 129 1970-01-01 01:00 vcsa1
crw------- 1 root   root     7,  64 1970-01-01 01:00 vcsu
crw------- 1 root   root     7,  65 1970-01-01 01:00 vcsu1
crw------- 1 root   root    81,   0 1970-01-01 01:00 video1
crw------- 1 root   root    81,   9 1970-01-01 01:00 video10
crw------- 1 root   root    81,  14 1970-01-01 01:00 video11
crw------- 1 root   root    81,   5 1970-01-01 01:00 video12
crw------- 1 root   root    81,  10 1970-01-01 01:00 video13
crw------- 1 root   root    81,  15 1970-01-01 01:00 video14
crw------- 1 root   root    81,  17 1970-01-01 01:00 video15
crw------- 1 root   root    81,  18 1970-01-01 01:00 video16
crw------- 1 root   root    81,   6 1970-01-01 01:00 video17
crw------- 1 root   root    81,  11 1970-01-01 01:00 video18
crw------- 1 root   root    81,  16 1970-01-01 01:00 video19
crw------- 1 root   root    81,   1 1970-01-01 01:00 video2
crw------- 1 root   root    81,  19 1970-01-01 01:00 video20
crw------- 1 root   root    81,   2 1970-01-01 01:00 video3
crw------- 1 root   root    81,   7 1970-01-01 01:00 video4
crw------- 1 root   root    81,  12 1970-01-01 01:00 video5
crw------- 1 root   root    81,   3 1970-01-01 01:00 video6
crw------- 1 root   root    81,   8 1970-01-01 01:00 video7
crw------- 1 root   root    81,  13 1970-01-01 01:00 video8
crw------- 1 root   root    81,   4 1970-01-01 01:00 video9
crw-rw-rw- 1 root   root    10, 102 1970-01-01 01:00 vndbinder
crw------- 1 root   root    10, 130 1970-01-01 01:00 watchdog
crw------- 1 root   root   247,   0 1970-01-01 01:00 watchdog0
crw------- 1 root   root    10,  63 2026-11-23 00:51 wcn
crw------- 1 root   root    10,  66 2026-11-23 00:51 wcn_op
-rw------- 1 root   root       1024 1970-01-01 01:00 wlm_shm_XXXXXXXXX
srwx------ 1 root   root          0 1970-01-01 01:00 wlm_shm_sync
crw-rw-rw- 1 root   root     1,   5 1970-01-01 01:00 zero
```

## Free space

```shell
$ df -h
Filesystem              Size  Used Avail Use% Mounted on
rootfs                  219M   10M  209M   5% /
devtmpfs                219M  344K  219M   1% /dev
tmpfs                   225M     0  225M   0% /mnt
/dev/block/dm-0         593M  547M   47M  93% /system
/dev/block/mmcblk0p15   138M   84M   54M  61% /vendor
/dev/block/mmcblk0p22   488M   29M  459M   6% /data
/dev/block/mmcblk0p21   3.8G  3.6G  163M  96% /blackbox
tmpfs                    32M   12K   32M   1% /tmp
/dev/block/mmcblk0p5     11M  1.0M   10M  10% /cali
/dev/block/mmcblk0p20   309M  1.0M  308M   1% /cache
/dev/block/mmcblk0p4     11M  416K   11M   4% /factory_data
/dev/block/mmcblk0p23   586M  275M  311M  47% /blackbox/upgrade
/dev/block/mmcblk0p24   142M  105M   36M  75% /mnt/media_rw/emulated
/dev/block/vold/sdcard0  59G  7.5G   52G  13% /mnt/media_rw/sdcard0
```

## Free memory

```shell
$ free -h
                total        used        free      shared     buffers
Mem:             450M        401M         48M        5.0M         30M
-/+ buffers/cache:           372M         78M
Swap:            150M        137M         13M
```

## USB Devices

```shell
$ lsusb -vvv
Bus 002 Port 001 Device 002: ID 2ca3:1020
Bus 001 Port 000 Device 001: ID 1d6b:0002
Bus 002 Port 000 Device 001: ID 1d6b:0003
```

## ID

```shell
$ id
uid=0(root) gid=0(root) groups=0(root),1004(input),1007(log),1011(adb),1015(sdcard_rw),1028(sdcard_r),3001(net_bt_admin),3002(net_bt),3003(inet),3006(net_bw_stats),3009(readproc),3011(uhid) context=u:r:su:s0
```

## Uname

```shell
/vendor $ uname -a
Linux localhost 5.4.123-rt59-g4d0cc6697013-ab38460 #4 SMP PREEMPT_RT Mon Oct 19 15:16:42 CST 2026 aarch64
```

## Shell

```shell
$ echo $SHELL
/system/bin/sh
```

## /proc/version

```shell
$ cat /proc/version
Linux version 5.4.123-rt59-g4d0cc6697013-ab38460 (jenkins@android-6and9-ci-18.04-slim-img-release-pxrbb) (real-aarch64-linux-android-gcc (GCC) 4.9.x 20150123 (prerelease), GNU ld (binutils-2.27-41d8fcb) 2.27.0.20170315) #4 SMP PREEMPT_RT Mon Oct 19 15:16:42 CST 2026
```

## /proc/cmdline

```shell
$ cat /proc/cmdline
earlycon=uart8250,mmio32,0xf8020000 no_console_suspend console=secS0 921600 user_debug=255 audit=1 mp_state=production  androidboot.hardware=e3t splashboot=zv900 coherent_pool=0x100000 kmemleak=off crashkernel=12M maxcpus=4 security=selinux androidboot.selinux=enforcing androidboot.veritymode=enforcing chip_sn=091f08333757384650 board_sn=7A7XP6R001005L     production_sn=753XP6T702HDP0       androidboot.slot_suffix=1 androidboot.android_dt_dir=/proc/device-tree/firmware_a/android/ androidboot.hw_version=04.04.01.00 ptn_panic=0x0002000000,0x0001000000 ptn_media=0x01c9400000,0x0008dcb000 androidboot.mode=normal fc_sn_addr=F2801000 androidboot.active=00010000 androidboot.verity=1 androidboot.secure_debug=0 ps_thres=3794,1    wifi_key=0ab023fc quiet @@@@@@@@@@@@@
```

## Uptime

```shell
$ cat /proc/uptime
4385.66 14285.77
```

## SELinux

```shell
$ getenforce
Permissive
```

## Partitions

```shell
$ cat /proc/partitions
major minor  #blocks  name

 254        0     153600 zram0
 179        0    7636800 mmcblk0
 179        1      16384 mmcblk0p1
 179        2      16384 mmcblk0p2
 179        3      16384 mmcblk0p3
 179        4      16384 mmcblk0p4
 179        5      16384 mmcblk0p5
 179        6      16384 mmcblk0p6
 179        7      16384 mmcblk0p7
 259        0      16384 mmcblk0p8
 259        1      16384 mmcblk0p9
 259        2      16384 mmcblk0p10
 259        3      32768 mmcblk0p11
 259        4      32768 mmcblk0p12
 259        5     638976 mmcblk0p13
 259        6     638976 mmcblk0p14
 259        7     147456 mmcblk0p15
 259        8     147456 mmcblk0p16
 259        9      16384 mmcblk0p17
 259       10       8192 mmcblk0p18
 259       11       8192 mmcblk0p19
 259       12     344064 mmcblk0p20
 259       13    4161536 mmcblk0p21
 259       14     524288 mmcblk0p22
 259       15     626688 mmcblk0p23
 259       16     145196 mmcblk0p24
 253        0     633944 dm-0
 179       24   62367744 mmcblk1
 179       25   62367712 mmcblk1p1
```

## /dev/block/by-name

```shell
$ ls -l /dev/block/by-name 2>/dev/null
total 0
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 blackbox -> /dev/block/mmcblk0p21
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 cache -> /dev/block/mmcblk0p20
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 cali -> /dev/block/mmcblk0p5
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 cali_raw1 -> /dev/block/mmcblk0p18
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 cali_raw2 -> /dev/block/mmcblk0p19
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 env -> /dev/block/mmcblk0p2
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 factory -> /dev/block/mmcblk0p4
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 factory_raw -> /dev/block/mmcblk0p17
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 gpt -> /dev/block/mmcblk0p1
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 media -> /dev/block/mmcblk0p24
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 meta_raw -> /dev/block/mmcblk0p6
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 normal -> /dev/block/mmcblk0p11
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 normal_2 -> /dev/block/mmcblk0p12
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 panic -> /dev/block/mmcblk0p3
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 scp -> /dev/block/mmcblk0p7
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 scp_2 -> /dev/block/mmcblk0p8
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 system -> /dev/block/mmcblk0p13
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 system_2 -> /dev/block/mmcblk0p14
lrwxrwxrwx 1 root root 20 1970-01-01 01:00 tos -> /dev/block/mmcblk0p9
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 tos_2 -> /dev/block/mmcblk0p10
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 upgrade -> /dev/block/mmcblk0p23
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 userdata -> /dev/block/mmcblk0p22
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 vendor -> /dev/block/mmcblk0p15
lrwxrwxrwx 1 root root 21 1970-01-01 01:00 vendor_2 -> /dev/block/mmcblk0p16
```

## /proc/cpuinfo

```shell$ cat /proc/cpuinfo
processor       : 0
BogoMIPS        : 7.68
Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32 atomics fphp asimdhp cpuid asimdrdm lrcpc dcpop asimddp
CPU implementer : 0x41
CPU architecture: 8
CPU variant     : 0x2
CPU part        : 0xd05
CPU revision    : 0

processor       : 1
BogoMIPS        : 7.68
Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32 atomics fphp asimdhp cpuid asimdrdm lrcpc dcpop asimddp
CPU implementer : 0x41
CPU architecture: 8
CPU variant     : 0x2
CPU part        : 0xd05
CPU revision    : 0

processor       : 2
BogoMIPS        : 7.68
Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32 atomics fphp asimdhp cpuid asimdrdm lrcpc dcpop asimddp
CPU implementer : 0x41
CPU architecture: 8
CPU variant     : 0x2
CPU part        : 0xd05
CPU revision    : 0

processor       : 3
BogoMIPS        : 7.68
Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32 atomics fphp asimdhp cpuid asimdrdm lrcpc dcpop asimddp
CPU implementer : 0x41
CPU architecture: 8
CPU variant     : 0x2
CPU part        : 0xd05
CPU revision    : 0
```

## /proc/meminfo

```shell
$ cat /proc/meminfo
MemTotal:         460436 kB
MemFree:           52724 kB
MemAvailable:     114460 kB
Buffers:           29308 kB
Cached:           116820 kB
SwapCached:          112 kB
Active:            73756 kB
Inactive:         157392 kB
Active(anon):      45480 kB
Inactive(anon):    44912 kB
Active(file):      28276 kB
Inactive(file):   112480 kB
Unevictable:       10804 kB
Mlocked:           10804 kB
SwapTotal:        153596 kB
SwapFree:           8444 kB
Dirty:              1496 kB
Writeback:             0 kB
AnonPages:         95672 kB
Mapped:            23020 kB
Shmem:              5188 kB
KReclaimable:      27064 kB
Slab:              95616 kB
SReclaimable:      27064 kB
SUnreclaim:        68552 kB
KernelStack:       18948 kB
PageTables:         7364 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:      383812 kB
Committed_AS:     184112 kB
VmallocTotal:   262930368 kB
VmallocUsed:       28368 kB
VmallocChunk:          0 kB
Percpu:              448 kB
CmaTotal:              0 kB
CmaFree:               0 kB
```

## /proc/modules

```shell

$ cat /proc/modules
sprdbt_tty 40960 0 - Live 0xffffffc008d69000 (O)
snd_usb_audio 274432 0 - Live 0xffffffc008d25000
uwe5622_bsp_sdio 245760 1 sprdbt_tty, Live 0xffffffc008ce8000 (O)
spi_slave_nrf52810_rx 290816 2 - Live 0xffffffc008ca0000
spi_nrf52810_rx 24576 2 - Live 0xffffffc008bf9000
hci_uart 36864 0 - Live 0xffffffc008bef000
bluetooth 581632 1 hci_uart, Live 0xffffffc008c11000
ecdh_generic 16384 1 bluetooth, Live 0xffffffc008b06000
ecc 36864 1 ecdh_generic, Live 0xffffffc008c07000
cfg80211 462848 0 - Live 0xffffffc008b7d000
snd_usbmidi_lib 36864 1 snd_usb_audio, Live 0xffffffc008b73000
snd_rawmidi 36864 1 snd_usbmidi_lib, Live 0xffffffc008b69000
snd_hwdep 20480 1 snd_usb_audio, Live 0xffffffc008b63000
snd_soc_dji_dummy_codec 16384 1 - Live 0xffffffc008b5e000
designware_i2s 28672 2 - Live 0xffffffc008b56000
snd_soc_simple_card 24576 0 - Live 0xffffffc008b4f000
snd_soc_simple_card_utils 24576 1 snd_soc_simple_card, Live 0xffffffc008b48000
dji_wl 16384 0 - Live 0xffffffc008adb000 (O)
vc_decoder 69632 0 - Live 0xffffffc008b36000 (O)
vc_encoder 40960 0 - Live 0xffffffc008b2b000 (O)
codec_h264e_e3t 126976 2 dji_wl, Live 0xffffffc008b0b000 (O)
codec_r2t_e3t 53248 1 - Live 0xffffffc008af8000 (O)
codec_cdma_e3t 61440 1 - Live 0xffffffc008ae2000 (O)
codec_h265e_e3t 147456 1 dji_wl, Live 0xffffffc008ab6000 (O)
codec_pm 24576 4 vc_decoder,vc_encoder,codec_h264e_e3t,codec_h265e_e3t, Live 0xffffffc008aab000 (O)
vc_encoder_pm_jpeg 24576 1 vc_encoder, Live 0xffffffc008aa0000 (O)
ask_dsp_driver 45056 2 - Live 0xffffffc008a90000 (O)
ask_dsp_pm 16384 1 ask_dsp_driver, Live 0xffffffc008a7f000 (O)
camecg_drv 16384 1 - Live 0xffffffc008a88000 (O)
icc_chnl 188416 8 - Live 0xffffffc008a50000 (O)
```

## IP Addresses

```shell
$ ip addr
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
2: rndis0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP group default qlen 1000
    link/ether 06:1d:6c:a4:31:b5 brd ff:ff:ff:ff:ff:ff
    inet 192.168.60.2/24 brd 192.168.60.255 scope global rndis0
       valid_lft forever preferred_lft forever
```

## IP Routes

```shell
$ ip route
192.168.60.0/24 dev rndis0 proto kernel scope link src 192.168.60.2 

/vendor $ cat /proc/net/tcp
  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode                                                     
   0: 0100007F:13AD 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 25902 1 00000000820544b7 100 0 0 10 0                     
   1: 00000000:0015 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 12624 1 000000005445c0b3 100 0 0 10 0                     
   2: 00000000:0587 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 25311 1 00000000996b5c32 100 0 0 10 0  



/vendor $ cat /proc/net/udp
  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops             
    4: 0100007F:1C6D 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 12144 2 00000000105a640f 0         
   26: 0100007F:D683 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 9064 2 00000000d10b7078 0          
  127: 0100FFEF:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16821 2 000000002b2401a5 0         
  127: 00000000:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16818 2 00000000c014bd13 0         
  127: 0100FFEF:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16577 2 00000000b0f4ac52 0         
  127: 00000000:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16574 2 00000000c0f12eca 0         
  127: 0100FFEF:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 12831 2 00000000423517b4 0         
  127: 00000000:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 12828 2 000000000db63e7d 0         
  127: 0100FFEF:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 9200 2 00000000bb0c7bd7 0          
  127: 00000000:1CE8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 9197 2 00000000f5cf737e 0          
  139: 00000000:1CF4 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 9046 2 00000000fbaa619f 0          
  140: 00000000:1CF5 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 9054 2 000000009fb22d6d 0          
  141: 00000000:1CF6 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 12640 2 0000000022cec6cf 0         
  142: 00000000:1CF7 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 12644 2 00000000718c5894 0         
  143: 00000000:1CF8 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16416 2 0000000043f351e9 0         
  144: 00000000:1CF9 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16421 2 00000000b62e5209 0         
  145: 00000000:1CFA 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16679 2 00000000bb5c65f3 0         
  146: 00000000:1CFB 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16694 2 000000001a79be36 0         
  194: 00000000:232B 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 13164 2 00000000a020fede 0         
  352: 0100007F:E7C9 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16426 2 00000000de0d6bec 0         
  373: 0100007F:D5DE 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 16699 2 00000000ad560ef6 0         
  430: 0100007F:B217 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 12657 2 0000000044582de0 0         
  449: 0100007F:1A2A 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 11490 2 0000000059fe136a 0         
  474: 00000000:0043 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 56525 2 000000000bd1555f 0         
```

## Logcat

```shell
$ logcat -d -t 300
--------- beginning of main
11-23 02:06:37.010  3258  3380 E SysMode : Waiting for service SysModeManagerService...
--------- beginning of system
11-23 02:06:37.011  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.011   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.011   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.012  3258  3341 I ServiceManager: Waiting for service RecorderService...
11-23 02:06:37.012  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.012   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.014  3258  3432 I DUSS4A  : EW-PLATFORM: value_5d: 1763
11-23 02:06:37.020   986  1149 E DUSS73  : wl_mgr: lk_state: wlm_link_state_manage_task(460): failed to get target dev, ret 7, wlm_link_state_manage_task
11-23 02:06:37.031   982  1349 E DUSS43  : [LINK_DEBUG-EVENT_SEND-WAIT_LIST] Timeout to wait ack. Detail: seq_id=0 message=0x00080069 with attr=0x00001040 from machine=0x0900 host=0x1c07 to machine=0x0000 host=0x0800 
11-23 02:06:37.031   982  1349 E DUSS85  : Report secondary liveview mode change msg send failed. ret = -1002
11-23 02:06:37.033  3258  3388 F DUSS70  : [app_utils] utilSendEventAsync() line 253: succeed to send v1 cmd async, peer_host 0x1f07, msg_id 0x00b5, seq_id 8888, msg_len 4, result 0
11-23 02:06:37.040  1008  1264 E DUSS6F  : gfsk_agent: dev_mgr: rf/rssi1/2[2,127,127]recv/pkt[0,119]f-m/freq/area/tx-area[7,2472,1,1]rc-sta[0]confl/sdr-t[0,0]miss[[73,40,39],[72,39,40],[4,40,40],[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0]].fm/area[7,1]rc[,0,0][0]
11-23 02:06:37.043  3258  3405 I DUSS70  : [app_utils] DisableVerticalTimer() line 235: GimbalManager: disable vertical timer: uav disconnected
11-23 02:06:37.058  3258  3308 E DUSS43  : [LINK_DEBUG-EVENT_SEND-WAIT_LIST] Timeout to wait ack. Detail: seq_id=6333 message=0x00000088 with attr=0x00001040 from machine=0x0900 host=0x1c00 to machine=0x0000 host=0x0801 
11-23 02:06:37.058  3258  3308 F DUSS70  : [app_utils] connect() line 62: DevNotify: register to 0x801 failed, result is -1002
11-23 02:06:37.058  3258  3308 F DUSS70  : [app_utils] nc_add() line 287: nc_add update life item 0/2, 6, 10, 0
11-23 02:06:37.059  3258  3308 E DUSS70  : [app_utils] countryCodeSync() line 195: GetCountrySync resp resp_buf = 0 68 69
11-23 02:06:37.059  3258  3308 I DUSS70  : [app_utils] createDDSSubscribeTask() line 60: DDSManager: linked 0, mIsDDSSubscribed 0, mGlassSubscribeDDSTask is idle
11-23 02:06:37.092   356   356 W ServiceManagement: Waited for hwservicemanager.ready for a second, waiting another...
11-23 02:06:37.098   986  1138 E DUSS73  : libwlm: failed to get master sync_time, is_sync_master 0
11-23 02:06:37.111   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.111  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.111  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.112   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.112   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.113  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.113   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.139   986  1125 I DUSS73  : wl_mgr: power_ctrl: wlm_modem_onoff_control(2379): recved modem onoff control, msg_ver 0, link_type 0, cmd_type 1, control_type 0, from 0x0e04
11-23 02:06:37.139   336   506 E DUSS6C  : report_pigeon_state_2_wlm ok: state = 0, by_wlm=0
11-23 02:06:37.152  3258  3312 I DUSS70  : [app_utils] nfzDataStateJudgement() line 811: nfz state: 0, fc nfz num: 0, nfz num: 0, max height: 0
11-23 02:06:37.158   976  1592 E DUSS5A  : sys_event_get_date: resp time to host :0x1902, 2026-11-23-2:6:37, GMT 1, result:0
11-23 02:06:37.182   976  1561 W DUSS41  : [LINK_DEBUG-MB_RECV-HAL] Get datagram fail -1007, in channel [13] with recv length 65536
11-23 02:06:37.183   976  1561 E DUSS41  : Receive message from bulk err. -1007
11-23 02:06:37.183   976  1561 F DUSS45  : linux_bulk_read as device,0,0,0,0,4 faild, disconnected
11-23 02:06:37.184   982  1545 W OMXMdcSource: [Graph19] mdcGetImage:1433 Buffer not ready, lost one frame. filled queue size 6 empty queue size 0
11-23 02:06:37.192   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(220): DUMP_LEVEL_3, ds_comm: libwlm 0x605e2e78f0, shm_ctx 0x605e2e7d88, shm_cfg 0x605e2e78f8
11-23 02:06:37.192   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(228): DUMP_LEVEL_3 msg_id: shm_info_update ,recv_cnt 0 ,recv_bytes 0 , freq 0 ,bitrate 0 bps
11-23 02:06:37.192   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(228): DUMP_LEVEL_3 msg_id: sdr_bw_ifram_recv ,recv_cnt 0 ,recv_bytes 0 , freq 0 ,bitrate 0 bps
11-23 02:06:37.192   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(228): DUMP_LEVEL_3 msg_id: sdr_ext_info ,recv_cnt 0 ,recv_bytes 0 , freq 0 ,bitrate 0 bps
11-23 02:06:37.192   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(228): DUMP_LEVEL_3 msg_id: sdr_link_info_recv ,recv_cnt 0 ,recv_bytes 0 , freq 0 ,bitrate 0 bps
11-23 02:06:37.192   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(228): DUMP_LEVEL_3 msg_id: iframe_recv ,recv_cnt 0 ,recv_bytes 0 , freq 0 ,bitrate 0 bps
11-23 02:06:37.193   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(228): DUMP_LEVEL_3 msg_id: shm_info_update_v2 ,recv_cnt 0 ,recv_bytes 0 , freq 0 ,bitrate 0 bps
11-23 02:06:37.193   986   986 I DUSS73  : wl_mgr: ds_comm: wlm_ds_common_dump(228): DUMP_LEVEL_3 msg_id: sdr_link_test_recv ,recv_cnt 0 ,recv_bytes 0 , freq 0 ,bitrate 0 bps
11-23 02:06:37.206   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $RD_RSSI, 845, 1074, 0
11-23 02:06:37.208  1027  2265 E DUSS43  : [LINK_DEBUG-EVENT_SEND-WAIT_LIST] Timeout to wait ack. Detail: seq_id=7439 message=0x00000081 with attr=0x00001040 from machine=0x0900 host=0x1c05 to machine=0x0000 host=0x0a01 
11-23 02:06:37.208  1027  2264 E DUSS43  : [LINK_DEBUG-EVENT_SEND-WAIT_LIST] Timeout to wait ack. Detail: seq_id=7439 message=0x00000081 with attr=0x00001040 from machine=0x0900 host=0x1c05 to machine=0x0000 host=0x0200 
11-23 02:06:37.209   976  2252 E DUSS0C  : [linux_gnss_sv_status_callback][243]
11-23 02:06:37.209   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $PCGDS,TSXUSE,0.000000,0.000000,0.000000,0.000000,*43
11-23 02:06:37.209   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $PCGDS,TCXO,963.426022,ppb/sec,*69
11-23 02:06:37.211   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.212  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.212  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.212   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.213   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.213  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.214   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_task(1080): ev_data 0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(638): scene_priority_cnt: L0 0, L1 0, L2 0, L3 0, L4 0, L5 0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(693): no scene care the agent SDR power_level, use default 1
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(696): SDR power_level: power_level 1 cacl_done 0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(693): no scene care the agent LTE power_level, use default 5
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(696): LTE power_level: power_level 5 cacl_done 0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(693): no scene care the agent WiFi power_level, use default 0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(696): WiFi power_level: power_level 0 cacl_done 0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(693): no scene care the agent GFSK power_level, use default 0
11-23 02:06:37.215   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_turn_scene(696): GFSK power_level: power_level 0 cacl_done 0
11-23 02:06:37.216   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_strip_lk_mode(492): link_mode: cmd SDR_ONLY, lv SDR_ONLY, download SDR_ONLY, comp_lk_mode SDR_ONLY, liveview_service 0 download_service 0, ext_ctrl_mask sdr 0, lte 0, wifi 0, gfsk: 0
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_strip_lk_mode(546): after mode_change: SDR 1 , LTE 5 , WIFI 0 GFSK 0, ext_ctrl_mask sdr 0, lte 0, wifi 0, gfsk 0, wlm_power_ctrl_strip_lk_mode
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_task(1214): power_level_cmd: sdr 1, lte 5, wifi 0, gfsk 0, external_pl: sdr 0, lte 0, wifi 0, gfsk 0, internal_pl: sdr 0, lte 0, wifi 0, gfsk 0, normal_pl: sdr 1, lte 5, wifi 0, gfsk 0, ext_ctrl_mask: sdr 0, lte 0, wifi 0, gfsk 0, int_ctrl_mask: sdr 0, lte 0, wifi 0, gfsk 0, ev_data 0, ctrl_all 1
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_sdrs_agent_v2(161): sdr_comm_set: version 0 , seq_id 0, ack_type 0, agent_type SDR, power_level 1 , scenario 0 , priv_len 36 
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_sdrs_agent_v2(168): sdr_set: sub_ver 0 , motor_status 0 , temp_status 0 , cam_work_mode 0 , distance 0.000000 , hight 0.000000 , speed 0.000000 , log_export 0, factory_status 0, allow_wifi_access 0
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_hdvt(262): sdr_comm_set: version 0 , seq_id 0, ack_type 0, agent_type SDR, power_level 1 , scenario 0 , priv_len 36 
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_hdvt(269): sdr_set: sub_ver 0 , motor_status 0 , temp_status 0 , cam_work_mode 0 , distance 0.000000 , hight 0.000000 , speed 0.000000 , log_export 0, factory_status 0, allow_wifi_access 0
11-23 02:06:37.217   986  1125 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_agent_ack(3121): ver 0, agt_type SDR, host_id 0xe04 seq_id 0, ack_sync 0, result 0, wlm_power_ctrl_set_agent_ack
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_gfsk(409): gfsk_comm_set: version 0 , seq_id 0, ack_type 0, agent_type GFSK, power_level 0 , scenario 0 , priv_len 9 
11-23 02:06:37.217   986  1151 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_gfsk(411): gfsk_set: sub_ver 0 
11-23 02:06:37.218   986  1125 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_agent_ack(3121): ver 0, agt_type SDR, host_id 0xe00 seq_id 0, ack_sync 0, result 0, wlm_power_ctrl_set_agent_ack
11-23 02:06:37.218  1008  1263 E DUSS6F  : gfsk_agent: dev_mgr: gfsk_dev_wlm_pwr_ctrl_set, recv msg from 0x0xe07, power level: 0
11-23 02:06:37.218  1008  1263 E DUSS6F  : gfsk_agent: dev_mgr: gfsk_dev_mgr_set_pwr_level, pwr level[0 -> 0]
11-23 02:06:37.218  1008  1263 E DUSS6F  : gfsk_agent: dev_mgr: gfsk_dev_mgr_op_rf, rf state[2 -> 2]
11-23 02:06:37.218   986  1125 I DUSS73  : wl_mgr: power_ctrl: wlm_power_ctrl_set_agent_ack(3121): ver 0, agt_type GFSK, host_id 0xe03 seq_id 0, ack_sync 0, result 0, wlm_power_ctrl_set_agent_ack
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPGGA,010637.000,5037.519143,N,00619.987360,E,1,07,5.433,618.629,M,0,M,,*72
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPRMC,010637.000,A,5037.519143,N,00619.987360,E,1.295,9.183,231126,,E,A*26
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPGLL,5037.519143,N,00619.987360,E,010637.000,A,A*5D
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPGSA,A,3,14,3,17,22,19,,,,,,,,6.614,5.433,3.772*0F
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $BDGSA,A,3,,,,,,,,,,,,,6.614,5.433,3.772*26
11-23 02:06:37.218   976  2252 E DUSS0C  : [linux_gps_location_callback][214]
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GAGSA,A,3,124,128,,,,,,,,,,,6.614,5.433,3.772*2A
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPGSV,4,1,16,14,7,259,13,3,64,252,10,17,41,298,22,22,15,280,32*7F
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPGSV,4,2,16,19,16,318,24,1,79,104,,2,54,121,,4,16,186,*4C
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPGSV,4,3,16,8,11,173,,28,14,81,,31,6,108,,32,22,43,*79
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPGSV,4,4,16,193,1,,,194,1,,,195,1,,,196,1,,*7A
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GAGSV,1,1,4,124,21,292,27,128,33,289,22,96,15,240,,130,12,330,*60
11-23 02:06:37.218   976  2251 I DUSS0C  : DJICallback::reportNmea: NMEA: $GPVTG,9.183,T,,M,1.295,N,2.399,K,A*00
11-23 02:06:37.220   986  1149 E DUSS73  : wl_mgr: lk_state: wlm_link_state_manage_task(460): failed to get target dev, ret 7, wlm_link_state_manage_task
11-23 02:06:37.287  2430  4056 I adbd    : read from usb: A_CNXN_num:9, A_OPEN_num:188, A_OKAY_num:10800, A_CLSE_num:187, A_WRTE_num:8745, A_DATA_num:8942
11-23 02:06:37.287  2430  4056 I adbd    : write to usb: A_CNXN_num:5, A_OPEN_num:0, A_OKAY_num:8932, A_CLSE_num:369, A_WRTE_num:10800, A_DATA_num:10805
11-23 02:06:37.287  3258  3301 E DUSS70  : [app_utils] handleDriverlicense() line 5852: FlyCtrlManager: country code is invalid!
11-23 02:06:37.289  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 6 not found
11-23 02:06:37.289  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 7 not found
11-23 02:06:37.289  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 8 not found
11-23 02:06:37.289  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 9 not found
11-23 02:06:37.289  2430  2430 I adbd    : starting raw subprocess (protocol=none, TERM=(null)): 'cd '/vendor' 2>/dev/null; clear
11-23 02:06:37.289  2430  2430 I adbd    : __t=$?; echo "__PWD:$(pwd)"'
11-23 02:06:37.292  2430  2430 I adbd    : subprocess creation successful: local_socket_fd=18, pid=11043
11-23 02:06:37.294  3258  3311 F DUSS70  : [app_utils] gohomeCheckTimer() line 1388: VisionManager: object_handler->mModemState is 6
11-23 02:06:37.312   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.312  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.314  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.314   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.314  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.314   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.315   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.315   976  1702 I DUSS86  : detect[0]need skip 0 0
11-23 02:06:37.324   982  1545 I OMXMdcSource: [Graph19] mdcGetImage:2050 [FPS] cnt 202440 timeuse = 1.338843 fps = 44.814816
11-23 02:06:37.324   982  1556 I OMXVC8000Enc: [Graph19] processBuffer:1399 Encoder::- input count 202440, true submitted frame count:202440, encoded Count:202434, resolution:1920*1080, metadata In/Out Count:0/0, FPS: 30000 / 1000 
11-23 02:06:37.341   982  1547 W DUSS26  : DISPSRVW: recycle_fb vop0: task_loop_times:453000
11-23 02:06:37.351   982  1547 W DUSS26  : DISPSRVW: recycle_fb vop0: vsync_success_get_times:453000
11-23 02:06:37.358   976  1592 E DUSS5A  : sys_event_get_date: resp time to host :0x1902, 2026-11-23-2:6:37, GMT 1, result:0
11-23 02:06:37.364   982  1545 W OMXMdcSource: [Graph19] mdcGetImage:1433 Buffer not ready, lost one frame. filled queue size 6 empty queue size 0
11-23 02:06:37.372   982  1317 I DUSS43  : Warning! ack timeout is less than monitor interval.
11-23 02:06:37.372   982  1317 E DUSS43  : Warning! ack timeout is less than monitor interval.
11-23 02:06:37.372   982  1317 I DUSS85  : Report errcode 0 result 0
11-23 02:06:37.403  3258  3319 F DUSS70  : [app_utils] handleSDRRole() line 1431: HdvtManager: msg_len 2, seq_id 40599, peer_host 0x0e00, msg_id 0x90043, msg_buf[0] 2
11-23 02:06:37.405  3258  3457 I DUSS5D  : gs_notification_ui_propose_item() line 2933: gs_notification_ui_propose_item code 12 event 1
11-23 02:06:37.405  3258  3460 I DUSS4A  : trace: "Device Post: set UIProposedItem_DeviceNotificationCenter"
11-23 02:06:37.405  3258  3457 F DUSS70  : [app_utils] nc_add() line 287: nc_add update life item 1/2, 8, c, 0
11-23 02:06:37.413   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.413  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.415  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.415   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.415   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.420   986  1149 E DUSS73  : wl_mgr: lk_state: wlm_link_state_manage_task(460): failed to get target dev, ret 7, wlm_link_state_manage_task
11-23 02:06:37.427  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.427   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.435   976  1735 I DUSS05  : set pwm 2 duty, set highlevel_ns: 24000
11-23 02:06:37.438   976  1735 I DUSS5A  : fan capture cnt[0:1]: period[3822510], cycle[1905230], duty[49], target[60], rps[130], rpm[7800]
11-23 02:06:37.454   982  1557 I OMXMdcSenderSink: [Graph19] processBuffer:1082 [FPS] mdc send frame cnt 202440 timeuse 1341ms fps 44.742729
11-23 02:06:37.475   982  1578 I OMXVC8000Enc: [Graph19] streamBufferFullProcess:487 On Receiving idr frame, append sps/pps
11-23 02:06:37.476   982  1578 I OMXVC8000Enc: [Graph19] addExtraData:338 OMXExtDatHlp:addExtraData():REMAIN: 1791022,EXTRA SIZE:39
11-23 02:06:37.500   982  1547 W DUSS26  : DISPSRVW: recycle_fb [vop0-plane_7]:total_pop_frames_cnt:226500
11-23 02:06:37.504   982  1541 I OMXMdcSenderSink: [Graph18] processBuffer:1082 [FPS] mdc send frame cnt 226500 timeuse 1199ms fps 50.041701
11-23 02:06:37.505  1008  1245 E DUSS6F  : gfsk_agent: link_mgr: gfsk_link_mgr_query_peer_rc_info
11-23 02:06:37.514   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.514  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.516  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.516   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.516   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.528  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.533  3258  3388 F DUSS70  : [app_utils] utilSendEventAsync() line 253: succeed to send v1 cmd async, peer_host 0x1f07, msg_id 0x00b5, seq_id 8889, msg_len 4, result 0
11-23 02:06:37.528   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.558   976  1592 E DUSS5A  : sys_event_get_date: resp time to host :0x1902, 2026-11-23-2:6:37, GMT 1, result:0
11-23 02:06:37.560  3258  3308 F DUSS70  : [app_utils] utilSendEventSync() line 288: succeed to send v1 cmd sync, peer_host 0x0e00, msg_id 0x6002f, seq_id 6336, msg_len 1, resp_len 2, resp_buf[0] 0
11-23 02:06:37.561  3258  3308 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 6 not found
11-23 02:06:37.561  3258  3308 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 7 not found
11-23 02:06:37.561  3258  3308 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 8 not found
11-23 02:06:37.561  3258  3308 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 9 not found
11-23 02:06:37.561  3258  3308 I DUSS70  : [app_utils] doLinkCheck() line 171: DevNotify: disconnect with 801 more than 5 seconds, reconnect
11-23 02:06:37.564   982  1545 W OMXMdcSource: [Graph19] mdcGetImage:1433 Buffer not ready, lost one frame. filled queue size 6 empty queue size 0
11-23 02:06:37.598   986  1138 E DUSS73  : libwlm: failed to get master sync_time, is_sync_master 0
11-23 02:06:37.614   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.615  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.616   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.616  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.616   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.620   986  1149 E DUSS73  : wl_mgr: lk_state: wlm_link_state_manage_task(460): failed to get target dev, ret 7, wlm_link_state_manage_task
11-23 02:06:37.628  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.628   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.640  1008  1264 E DUSS6F  : gfsk_agent: dev_mgr: rf/rssi1/2[2,127,127]recv/pkt[0,120]f-m/freq/area/tx-area[7,2404,1,1]rc-sta[0]confl/sdr-t[0,0]miss[[73,40,40],[72,40,40],[4,40,40],[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0]].fm/area[7,1]rc[,0,0][0]
11-23 02:06:37.680  3258  3409 W DUSS26  : DISPSRVW: recycle_fb vop0: task_loop_times:444600
11-23 02:06:37.683   976  1561 W DUSS41  : [LINK_DEBUG-MB_RECV-HAL] Get datagram fail -1007, in channel [13] with recv length 65536
11-23 02:06:37.683   976  1561 E DUSS41  : Receive message from bulk err. -1007
11-23 02:06:37.683   976  1561 F DUSS45  : linux_bulk_read as device,0,0,0,0,4 faild, disconnected
11-23 02:06:37.690  3258  3409 W DUSS26  : DISPSRVW: recycle_fb vop0: vsync_success_get_times:444600
11-23 02:06:37.691  3258  3365 I DUSS70  : [app_utils] checkGlassLocalUSBCurrentRole() line 294: IntegratedBatteryManager: otg_state_str peripheral, state CONFIGURED
11-23 02:06:37.693   976   976 E DUSS5A  : MONITOR: cpu general loading: 13.506%
11-23 02:06:37.694   976   976 I DUSS5A  : INFO: *******sys_status:0x00*******
11-23 02:06:37.708   976  1558 E DUSS46  : usbpri2_slave_send() line 2219: usbpri2 error: send data fail, -1/19. len: 77, timeout: 150, in04
11-23 02:06:37.708   976  1558 F DUSS45  : linux_bulk_write as device,0,0,0,0,4 faild, disconnected
11-23 02:06:37.708   976  1558 F DUSS41  : [LINK_DEBUG-MB_FORWARD-N/A] Router error -1007 on host 1c01. Detail: seq_id=7441 message=0x00000081 with attr=0x00001040 from machine=0x0000 host=0x1c05 to machine=0x0000 host=0x0a01 
11-23 02:06:37.715   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.716  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.717   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.717  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.717   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.729  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.729   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.744   982  1545 W OMXMdcSource: [Graph19] mdcGetImage:1433 Buffer not ready, lost one frame. filled queue size 6 empty queue size 0
11-23 02:06:37.758   976  1592 E DUSS5A  : sys_event_get_date: resp time to host :0x1902, 2026-11-23-2:6:37, GMT 1, result:0
11-23 02:06:37.761  3258  3308 E DUSS43  : [LINK_DEBUG-EVENT_SEND-WAIT_LIST] Timeout to wait ack. Detail: seq_id=6334 message=0x00000088 with attr=0x00001040 from machine=0x0900 host=0x1c00 to machine=0x0000 host=0x0801 
11-23 02:06:37.761  3258  3308 F DUSS70  : [app_utils] connect() line 62: DevNotify: register to 0x801 failed, result is -1002
11-23 02:06:37.761  3258  3308 F DUSS70  : [app_utils] nc_add() line 287: nc_add update life item 0/2, 6, 10, 0
11-23 02:06:37.762  3258  3308 E DUSS70  : [app_utils] countryCodeSync() line 195: GetCountrySync resp resp_buf = 0 68 69
11-23 02:06:37.762  3258  3308 I DUSS70  : [app_utils] createDDSSubscribeTask() line 60: DDSManager: linked 0, mIsDDSSubscribed 0, mGlassSubscribeDDSTask is idle
11-23 02:06:37.771   976  2275 I DUSS0C  : [gps_get_status][443]
11-23 02:06:37.771   976  2275 I DUSS5A  : [ZV902] GNSS: get status: 1
11-23 02:06:37.771   976  2275 I DUSS0C  : [gps_get_gnss_sv_status][458]
11-23 02:06:37.771   976  2275 I DUSS5A  : [ZV902] GNSS: num_svs 7
11-23 02:06:37.771   976  2275 I DUSS5A  : [ZV902] GNSS: index:0, svid:14, constellation:1, c_n0_dbhz:13, elevation:7.832444, zimuth:259.384369, flags:15, carrier_freq:1575420032.000000
11-23 02:06:37.771   976  2275 I DUSS5A  : [ZV902] GNSS: index:1, svid:3, constellation:1, c_n0_dbhz:10, elevation:64.372574, zimuth:252.521378, flags:15, carrier_freq:1575420032.000000
11-23 02:06:37.771   976  2275 I DUSS5A  : [ZV902] GNSS: index:2, svid:17, constellation:1, c_n0_dbhz:22, elevation:41.074894, zimuth:298.234131, flags:15, carrier_freq:1575420032.000000
11-23 02:06:37.771   976  2275 I DUSS5A  : [ZV902] GNSS: index:3, svid:22, constellation:1, c_n0_dbhz:32, elevation:15.288261, zimuth:280.357941, flags:15, carrier_freq:1575420032.000000
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: index:4, svid:19, constellation:1, c_n0_dbhz:24, elevation:16.496464, zimuth:318.281555, flags:15, carrier_freq:1575420032.000000
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: index:5, svid:30, constellation:6, c_n0_dbhz:27, elevation:21.841013, zimuth:292.959106, flags:15, carrier_freq:1575420032.000000
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: index:6, svid:34, constellation:6, c_n0_dbhz:22, elevation:33.673779, zimuth:289.818085, flags:15, carrier_freq:1575420032.000000
11-23 02:06:37.772   976  2275 I DUSS0C  : [gps_get_location][419]
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: latitude:  50.625319
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: longtitude: 6.333123
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: altitude: 618.629000
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: timestamp: 1795395997000
11-23 02:06:37.772   976  2275 I DUSS5A  : [ZV902] GNSS: flags: 255
11-23 02:06:37.773   976  2275 I DUSS5A  : [ZV902] GNSS: send location info to 0x0300, msg length: 84 success
11-23 02:06:37.773   976  2275 I DUSS5A  : [ZV902] GNSS: send location info to 0x0300, msg length: 13 success
11-23 02:06:37.773   976  2275 I DUSS5A  : [ZV902] GNSS: send location info to 0x1c00, msg length: 84 success
11-23 02:06:37.789  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 6 not found
11-23 02:06:37.789  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 7 not found
11-23 02:06:37.789  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 8 not found
11-23 02:06:37.789  3258  3303 F DUSS70  : [app_utils] nc_remove() line 348: nc_remove item c, 9 not found
11-23 02:06:37.796  3258  3432 I DUSS4A  : EW-PLATFORM: value_5d: 1764
11-23 02:06:37.816   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.816  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.817   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.817  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.818   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.820   986  1149 E DUSS73  : wl_mgr: lk_state: wlm_link_state_manage_task(460): failed to get target dev, ret 7, wlm_link_state_manage_task
11-23 02:06:37.829  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.830   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.870   976  1592 E DUSS5A  : plat_usb_otg_event_callback enter!
11-23 02:06:37.870   976  1592 E DUSS5A  : plat_usb_otg_event_callback enter!
11-23 02:06:37.870   976  1855 E DUSS5A  : peer_host:0x1902
11-23 02:06:37.870  3258  3319 I DUSS70  : [app_utils] handleCmdReq() line 81: Push DUSS_MB_CMD_BATTERY_GET_STATIC_INFO begin
11-23 02:06:37.870   976  1855 E DUSS5A  :  otg_mode:0x02 from M7
11-23 02:06:37.870   976  1855 E DUSS5A  : peer_host:0x1902
11-23 02:06:37.870   976  1855 E DUSS5A  :  otg_mode:0x02 from M7
11-23 02:06:37.870  3258  3319 I DUSS70  : [app_utils] handleGetBatteryInfoCb() line 71: IntegratedBatteryManager: get battery dynamic info: voltage[8282mv], percent[100], temp[230]
11-23 02:06:37.870  3258  3319 E DUSS70  : [app_utils] checkBatteryStability() line 113: IntegratedBatteryManager: max-min = 0.000000, BatteryVoltageStatus = 2
11-23 02:06:37.870  3258  3319 I DUSS70  : [app_utils] updateBatteryUsageInfo() line 92: IntegratedBatteryManager: update battery usage info: voltage[8.282001V], percent[100], auth_type[1],charge_state[0]
11-23 02:06:37.870  3258  3319 I DUSS70  : [app_utils] getCurrentGlassType() line 643: ProductManager: current glass type is 3
11-23 02:06:37.870  3258  3319 I DUSS70  : [app_utils] pushBatteryInfo() line 241: IntegratedBatteryManager: push battery info to ew, voltage[8.282001V], percent[100], auth_type[1], tmp_warning[0], battery_status[1],charge_state[1]
11-23 02:06:37.871  3258  3460 I DUSS4A  : trace: "DuOSD::GlassBatteryIndicator.onHandleVoltageEvent 1: voltage ", 8.282001, ", lowVoltageCounter ", 0, ", lowVoltageHitCondition ", 5, ", lowVoltageThreshold ", 6.280000, ", low battery strategy ", set:0x00000003
11-23 02:06:37.871  3258  3460 I DUSS4A  : trace: "currentChargeStatus: ", 1, "batteryPercent: ", 100
11-23 02:06:37.872   982  1317 I DUSS43  : Warning! ack timeout is less than monitor interval.
11-23 02:06:37.873   982  1317 E DUSS43  : Warning! ack timeout is less than monitor interval.
11-23 02:06:37.873   982  1317 I DUSS85  : Report errcode 0 result 0
11-23 02:06:37.871  3258  3460 I DUSS4A  : trace: "currentChargeStatus: ", 1, "batteryPercent: ", 100
11-23 02:06:37.903  3258  3319 F DUSS70  : [app_utils] handleSDRRole() line 1431: HdvtManager: msg_len 2, seq_id 40619, peer_host 0x0e00, msg_id 0x90043, msg_buf[0] 2
11-23 02:06:37.916  3258  3432 I DUSS4A  : EW-PLATFORM: value_5d: 1763
11-23 02:06:37.917   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.917  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:37.918   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.918   357   357 W ServiceManagement: Waited for hwservicemanager.ready for a second, waiting another...
11-23 02:06:37.918  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:37.918   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.920   982  1549 W DUSS26  : DISPSRVW: last 2018ms plane7 get 101 frame 50.05fps
11-23 02:06:37.923   982  1556 E OMXVC8000Enc: [Graph19] processBuffer:1188 out Buffer is NULL....
11-23 02:06:37.930  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:37.930   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:37.943   982  1545 W OMXMdcSource: [Graph19] mdcGetImage:1433 Buffer not ready, lost one frame. filled queue size 6 empty queue size 0
11-23 02:06:37.945   976  1425 I DUSS46  : Iap2_UsbDeviceTransport_IsConnect() line 195: Iap2 isConnect: False
11-23 02:06:37.958  3258  3319 I DUSS70  : [app_utils] handleCmdReq() line 189: handle DUSS_MB_CMD_QUERY_DEVICE_CAPS begin
11-23 02:06:37.958  3258  3319 I DUSS70  : [app_utils] handleDeviceInfoReq() line 1610: DevNotify: dn command type 25
11-23 02:06:37.958  3258  3319 I DUSS70  : [app_utils] ping() line 207: DevNotify: ping host 1c01!, result:0
11-23 02:06:37.958  3258  3319 I DUSS70  : [app_utils] handleCmdReq() line 191: handle DUSS_MB_CMD_QUERY_DEVICE_CAPS end
11-23 02:06:37.958   976  1592 E DUSS5A  : sys_event_get_date: resp time to host :0x1902, 2026-11-23-2:6:37, GMT 1, result:0
11-23 02:06:37.958   976  2012 F DUSS5A  : HOST: host v1_device_49 is online, retry times: 5
11-23 02:06:37.978  1215  1215 I DUSS7F  : =======send boot_mode to e3tm7,boot_mode:2========
11-23 02:06:37.998  1215  1215 I DUSS7F  : send boot_mode to e3tm7 success, boot_mode: 2
11-23 02:06:37.998  1215  1215 E DUSS7F  : year:2026,month:11,day:23,hour:2,minute:6,second:37
11-23 02:06:37.998  3258  3319 I DUSS70  : [app_utils] handleCmdReq() line 44: handle DUSS_MB_CMD_PING begin,peer host: 0x0500
11-23 02:06:37.998  3258  3319 E DUSS41  : [field_router] field route table query failed -1003! src: 1c00, tar: 0500, msg_id:00000000
11-23 02:06:37.998  3258  3319 E DUSS41  : [field_router] field route query failed when send msg -1003
11-23 02:06:37.998  3258  3319 E DUSS41  : [LINK_DEBUG-MB_FORWARD-TARGET-TABLE] Can't find target rt table when send on host 1c00. Detail: seq_id=2219 message=0x00000000 with attr=0x000000c0 from machine=0x0000 host=0x1c00 to machine=0x0000 host=0x0500 
11-23 02:06:37.998  3258  3319 I DUSS70  : [app_utils] handleCmdReq() line 50: handle DUSS_MB_CMD_PING end
11-23 02:06:37.998  3258  3319 E DUSS43  : [LINK_DEBUG-EVENT_HANDLE-N/A] Handle event error: -1007. Detail: seq_id=2219, msg_len=0, message=0x00000000 with attr=0x00001040 from machine=0x0000 host=0x0500 to machine=0x0900 host=0x1c00 
11-23 02:06:38.017   351   351 E SELinux : avc:  denied  { find } for service=SysModeManagerService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:38.017  3258  3380 E SysMode : Waiting for service SysModeManagerService...
11-23 02:06:38.018   351   351 E SELinux : avc:  denied  { find } for service=RecorderService pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:38.018  3258  3341 I ServiceManager: Waiting for service RecorderService...
11-23 02:06:38.018  3258  3437 I DUSS4A  : EwAddons<VideoReceiver>: mdc:connecting...
11-23 02:06:38.019   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:38.021   986  1149 E DUSS73  : wl_mgr: lk_state: wlm_link_state_manage_task(460): failed to get target dev, ret 7, wlm_link_state_manage_task
11-23 02:06:38.031  3258  3455 I DUSS4A  : EwAddon<Gfx-Sender>: connecting...
11-23 02:06:38.031   351   351 E SELinux : avc:  denied  { find } for service=mediadatacenter pid=3258 uid=0 scontext=u:r:su:s0 tcontext=u:object_r:default_android_service:s0 tclass=service_manager permissive=0
11-23 02:06:38.031   982  1349 I DUSS43  : Warning! ack timeout is less than monitor interval.
11-23 02:06:38.032   982  1349 E DUSS43  : Warning! ack timeout is less than monitor interval.
11-23 02:06:38.033  3258  3388 F DUSS70  : [app_utils] utilSendEventAsync() line 253: succeed to send v1 cmd async, peer_host 0x1f07, msg_id 0x00b5, seq_id 8890, msg_len 4, result 0
11-23 02:06:38.038  2430  2430 I adbd    : starting raw subprocess (protocol=none, TERM=(null)): 'cd '/vendor' 2>/dev/null; logcat -d -t 300
11-23 02:06:38.038  2430  2430 I adbd    : __t=$?; echo "__PWD:$(pwd)"'
11-23 02:06:38.041  2430  2430 I adbd    : subprocess creation successful: local_socket_fd=18, pid=11048
11-23 02:06:38.043  3258  3405 I DUSS70  : [app_utils] DisableVerticalTimer() line 235: GimbalManager: disable vertical timer: uav disconnected
11-23 02:06:38.054 11048 11048 I sh      : type=1400 audit(0.0:177): avc: denied { getattr } for path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
11-23 02:06:38.055 11048 11048 I sh      : type=1400 audit(0.0:178): avc: denied { execute } for name="logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
11-23 02:06:38.055 11050 11050 I sh      : type=1400 audit(0.0:179): avc: denied { read open } for path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
11-23 02:06:38.056 11050 11050 I sh      : type=1400 audit(0.0:179): avc: denied { execute_no_trans } for path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
11-23 02:06:38.060 11050 11050 I logcat  : type=1400 audit(0.0:179): avc: denied { map } for path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
11-23 02:06:38.082   982  1349 E DUSS43  : [LINK_DEBUG-EVENT_SEND-WAIT_LIST] Timeout to wait ack. Detail: seq_id=0 message=0x00080069 with attr=0x00001040 from machine=0x0900 host=0x1c07 to machine=0x0000 host=0x0800 
11-23 02:06:38.082   982  1349 E DUSS85  : Report secondary liveview mode change msg send failed. ret = -1002
11-23 02:06:38.084 11050 11050 I logcat  : type=1400 audit(0.0:180): avc: denied { write } for name="logdr" dev="devtmpfs" ino=2466 scontext=u:r:su:s0 tcontext=u:object_r:logdr_socket:s0 tclass=sock_file permissive=1
11-23 02:06:38.086 11050 11050 I logcat  : type=1400 audit(0.0:180): avc: denied { connectto } for path="/dev/socket/logdr" scontext=u:r:su:s0 tcontext=u:r:logd:s0 tclass=unix_stream_socket permissive=1
```

## Kernel Log

```shell
$ dmesg | tail -n 300
[ 4252.445271] (unrd) unrd_get_item Pid: 10491, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4254.568402] (kworker/u8:6) WCN BASE: info: tx:at+loopcheck=4254562,15678\x0d\x0a
[ 4254.569151] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4254562,cp2_bootup=15678,cp2_send=4254074\x0a
[ 4257.981629] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:02:01.515 boot time us:4257979150
[ 4257.981653] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:02:01.515 boot time us:4257979150
[ 4259.608517] (kworker/u8:6) WCN BASE: info: tx:at+loopcheck=4259602,15678\x0d\x0a
[ 4259.609228] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4259602,cp2_bootup=15678,cp2_send=4259113\x0a
[ 4264.414513] (unrd) unrd_get_item Pid: 10510, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4264.647255] (kworker/u8:6) WCN BASE: info: tx:at+loopcheck=4264642,15678\x0d\x0a
[ 4264.647926] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4264642,cp2_bootup=15678,cp2_send=4264151\x0a
[ 4267.307472] (sh) send_signal sig=1 current->comm=sh current->tgid=10518 t->comm=sh t->tgid=10518
[ 4267.911980] (sh) send_signal sig=1 current->comm=sh current->tgid=10523 t->comm=sh t->tgid=10523
[ 4267.984442] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:02:11.518 boot time us:4267981969
[ 4268.473901] (spc_manager) INFO, path: /blackbox, size: 3762.84MB -> 3774.85MB, rate: 24.0234MB/min
[ 4269.688596] (kworker/u8:6) WCN BASE: info: tx:at+loopcheck=4269682,15678\x0d\x0a
[ 4269.689292] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4269682,cp2_bootup=15678,cp2_send=4269192\x0a
[ 4274.726845] (kworker/u8:6) WCN BASE: info: tx:at+loopcheck=4274722,15678\x0d\x0a
[ 4274.727497] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4274722,cp2_bootup=15678,cp2_send=4274230\x0a
[ 4275.514651] (dcs_server) unrd_set_item Pid: 998, Opt: Write, Size: 39944, Partition: /dev/block/by-name/cali_raw1 
[ 4276.388333] (unrd) unrd_get_item Pid: 10537, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4277.987115] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:02:21.521 boot time us:4277984633
[ 4279.766863] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4279762,15678\x0d\x0a
[ 4279.767552] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4279762,cp2_bootup=15678,cp2_send=4279269\x0a
[ 4281.383214] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10547
[ 4281.383807] (sh) handle_signal sig=1 current->comm=sh current->tgid=10547
[ 4281.384770] (sh) send_signal sig=1 current->comm=sh current->tgid=10547 t->comm=sh t->tgid=10547
[ 4284.807249] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4284802,15678\x0d\x0a
[ 4284.807953] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4284802,cp2_bootup=15678,cp2_send=4284309\x0a
[ 4287.990305] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:02:31.524 boot time us:4287987830
[ 4288.357412] (unrd) unrd_get_item Pid: 10560, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4289.857987] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4289852,15678\x0d\x0a
[ 4289.858645] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4289852,cp2_bootup=15678,cp2_send=4289359\x0a
[ 4294.887114] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4294882,15678\x0d\x0a
[ 4294.887764] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4294882,cp2_bootup=15678,cp2_send=4294388\x0a
[ 4295.527547] (sh) send_signal sig=1 current->comm=sh current->tgid=10574 t->comm=sh t->tgid=10574
[ 4297.992899] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:02:41.526 boot time us:4297990444
[ 4297.992967] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:02:41.526 boot time us:4297990526
[ 4298.493531] (spc_manager) INFO, path: /blackbox, size: 3774.85MB -> 3788.09MB, rate: 26.4844MB/min
[ 4299.928464] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4299922,15678\x0d\x0a
[ 4299.929128] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4299922,cp2_bootup=15678,cp2_send=4299429\x0a
[ 4300.329629] (unrd) unrd_get_item Pid: 10583, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4304.968565] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4304962,15678\x0d\x0a
[ 4304.969298] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4304962,cp2_bootup=15678,cp2_send=4304468\x0a
[ 4305.676804] (sys_time_record)  Mon Nov 23 02:02:49 GMT 2026 GMT+1
[ 4307.995721] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:02:51.529 boot time us:4307993246
[ 4310.008089] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4310002,15678\x0d\x0a
[ 4310.008758] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4310002,cp2_bootup=15678,cp2_send=4309507\x0a
[ 4311.382282] (sh) send_signal sig=1 current->comm=sh current->tgid=10600 t->comm=sh t->tgid=10600
[ 4312.300692] (unrd) unrd_get_item Pid: 10603, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4315.057636] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4315052,15678\x0d\x0a
[ 4315.058300] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4315052,cp2_bootup=15678,cp2_send=4314556\x0a
[ 4317.998651] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:03:01.532 boot time us:4317996146
[ 4320.088607] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4320082,15678\x0d\x0a
[ 4320.089270] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4320082,cp2_bootup=15678,cp2_send=4319587\x0a
[ 4324.272676] (unrd) unrd_get_item Pid: 10623, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4325.128329] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4325122,15678\x0d\x0a
[ 4325.129035] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4325122,cp2_bootup=15678,cp2_send=4324626\x0a
[ 4325.995653] (logd.auditd) type=1400 audit(1795395592.279:172): avc: denied { open } for pid=10241 comm="cat" path="/verity_key" dev="rootfs" ino=1109 scontext=u:r:su:s0 tcontext=u:object_r:rootfs:s0 tclass=file permissive=1
[ 4325.995687] (logd.auditd) type=1400 audit(1795395789.519:173): avc: denied { read } for pid=10632 comm="cat" name="version" dev="proc" ino=4026532174 scontext=u:r:su:s0 tcontext=u:object_r:proc_version:s0 tclass=file permissive=1
[ 4325.996512] (logd.auditd) type=1400 audit(1795395789.519:173): avc: denied { read } for pid=10632 comm="cat" name="version" dev="proc" ino=4026532174 scontext=u:r:su:s0 tcontext=u:object_r:proc_version:s0 tclass=file permissive=1
[ 4325.996551] (logd.auditd) type=1400 audit(1795395789.519:173): avc: denied { open } for pid=10632 comm="cat" path="/proc/version" dev="proc" ino=4026532174 scontext=u:r:su:s0 tcontext=u:object_r:proc_version:s0 tclass=file permissive=1
[ 4326.001714] (sh) send_signal sig=1 current->comm=sh current->tgid=10630 t->comm=sh t->tgid=10630
[ 4328.001421] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:03:11.535 boot time us:4327998967
[ 4328.001429] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:03:11.535 boot time us:4327998991
[ 4328.511979] (spc_manager) INFO, path: /blackbox, size: 3788.09MB -> 3791.66MB, rate: 7.1406MB/min
[ 4330.168576] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4330162,15678\x0d\x0a
[ 4330.169266] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4330162,cp2_bootup=15678,cp2_send=4329666\x0a
[ 4335.207895] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4335202,15678\x0d\x0a
[ 4335.208610] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4335202,cp2_bootup=15678,cp2_send=4334705\x0a
[ 4336.242894] (unrd) unrd_get_item Pid: 10646, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4337.530607] (dcs_server) unrd_set_item Pid: 998, Opt: Write, Size: 39944, Partition: /dev/block/by-name/cali_raw1 
[ 4338.004273] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:03:21.538 boot time us:4338001767
[ 4340.257590] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4340252,15678\x0d\x0a
[ 4340.258297] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4340252,cp2_bootup=15678,cp2_send=4339754\x0a
[ 4340.716219] (sh) send_signal sig=1 current->comm=sh current->tgid=10655 t->comm=sh t->tgid=10655
[ 4341.379890] (sh) send_signal sig=1 current->comm=sh current->tgid=10660 t->comm=sh t->tgid=10660
[ 4345.297628] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4345292,15678\x0d\x0a
[ 4345.298317] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4345292,cp2_bootup=15678,cp2_send=4344793\x0a
[ 4348.007112] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:03:31.541 boot time us:4348004631
[ 4348.213369] (unrd) unrd_get_item Pid: 10671, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4350.327899] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4350322,15678\x0d\x0a
[ 4350.328552] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4350322,cp2_bootup=15678,cp2_send=4349823\x0a
[ 4355.367194] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4355362,15678\x0d\x0a
[ 4355.367854] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4355362,cp2_bootup=15678,cp2_send=4354862\x0a
[ 4358.009992] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:03:41.543 boot time us:4358007525
[ 4358.009993] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:03:41.543 boot time us:4358007525
[ 4358.531383] (spc_manager) INFO, path: /blackbox, size: 3791.66MB -> 3783.84MB, rate: -15.6406MB/min
[ 4360.185516] (unrd) unrd_get_item Pid: 10690, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4360.417936] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4360412,15678\x0d\x0a
[ 4360.418602] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4360412,cp2_bootup=15678,cp2_send=4359912\x0a
[ 4365.458010] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4365452,15678\x0d\x0a
[ 4365.458678] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4365452,cp2_bootup=15678,cp2_send=4364951\x0a
[ 4366.203729] (sys_time_record)  Mon Nov 23 02:03:49 GMT 2026 GMT+1
[ 4368.012896] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:03:51.546 boot time us:4368010379
[ 4370.498086] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4370492,15678\x0d\x0a
[ 4370.498827] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4370492,cp2_bootup=15678,cp2_send=4369991\x0a
[ 4371.383626] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10707
[ 4371.384652] (sh) send_signal sig=1 current->comm=sh current->tgid=10707 t->comm=sh t->tgid=10707
[ 4372.154455] (unrd) unrd_get_item Pid: 10710, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4375.528318] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4375522,15678\x0d\x0a
[ 4375.529690] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4375522,cp2_bootup=15678,cp2_send=4375021\x0a
[ 4378.015803] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:04:01.549 boot time us:4378013322
[ 4380.566661] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4380562,15678\x0d\x0a
[ 4380.567339] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4380562,cp2_bootup=15678,cp2_send=4380058\x0a
[ 4384.122696] (unrd) unrd_get_item Pid: 10729, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4385.227251] (sh) send_signal sig=1 current->comm=sh current->tgid=10735 t->comm=sh t->tgid=10735
[ 4385.607400] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4385602,15678\x0d\x0a
[ 4385.608207] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4385602,cp2_bootup=15678,cp2_send=4385099\x0a
[ 4385.670938] (logd.auditd) type=1400 audit(1795395789.519:173): avc: denied { open } for pid=10632 comm="cat" path="/proc/version" dev="proc" ino=4026532174 scontext=u:r:su:s0 tcontext=u:object_r:proc_version:s0 tclass=file permissive=1
[ 4385.670973] (logd.auditd) type=1400 audit(1795395849.189:174): avc: denied { read } for pid=10741 comm="cat" name="uptime" dev="proc" ino=4026532173 scontext=u:r:su:s0 tcontext=u:object_r:proc_uptime:s0 tclass=file permissive=1
[ 4385.671521] (logd.auditd) type=1400 audit(1795395849.189:174): avc: denied { read } for pid=10741 comm="cat" name="uptime" dev="proc" ino=4026532173 scontext=u:r:su:s0 tcontext=u:object_r:proc_uptime:s0 tclass=file permissive=1
[ 4385.671548] (logd.auditd) type=1400 audit(1795395849.189:174): avc: denied { open } for pid=10741 comm="cat" path="/proc/uptime" dev="proc" ino=4026532173 scontext=u:r:su:s0 tcontext=u:object_r:proc_uptime:s0 tclass=file permissive=1
[ 4385.677961] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10739
[ 4385.678390] (sh) send_signal sig=1 current->comm=sh current->tgid=10739 t->comm=sh t->tgid=10739
[ 4388.018417] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:04:11.552 boot time us:4388015949
[ 4388.018544] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:04:11.552 boot time us:4388016032
[ 4388.549662] (spc_manager) INFO, path: /blackbox, size: 3783.84MB -> 3795.86MB, rate: 24.0391MB/min
[ 4390.648217] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4390642,15678\x0d\x0a
[ 4390.648959] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4390642,cp2_bootup=15678,cp2_send=4390139\x0a
[ 4395.688013] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4395682,15678\x0d\x0a
[ 4395.688748] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4395682,cp2_bootup=15678,cp2_send=4395178\x0a
[ 4396.098170] (unrd) unrd_get_item Pid: 10756, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4398.021366] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:04:21.555 boot time us:4398018889
[ 4399.347749] (dcs_server) unrd_set_item Pid: 998, Opt: Write, Size: 39944, Partition: /dev/block/by-name/cali_raw1 
[ 4400.190614] (sh) send_signal sig=1 current->comm=sh current->tgid=10764 t->comm=sh t->tgid=10764
[ 4400.728560] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4400722,15678\x0d\x0a
[ 4400.729268] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4400722,cp2_bootup=15678,cp2_send=4400218\x0a
[ 4401.382314] (sh) send_signal sig=1 current->comm=sh current->tgid=10769 t->comm=sh t->tgid=10769
[ 4405.766579] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4405762,15678\x0d\x0a
[ 4405.767233] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4405762,cp2_bootup=15678,cp2_send=4405256\x0a
[ 4408.024204] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:04:31.558 boot time us:4408021689
[ 4408.024212] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:04:31.558 boot time us:4408021725
[ 4408.069900] (unrd) unrd_get_item Pid: 10780, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4410.808587] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4410802,15678\x0d\x0a
[ 4410.809246] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4410802,cp2_bootup=15678,cp2_send=4410297\x0a
[ 4411.465713] (sh) send_signal sig=1 current->comm=sh current->tgid=10788 t->comm=sh t->tgid=10788
[ 4411.466010] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10788
[ 4415.848203] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4415842,15678\x0d\x0a
[ 4415.848947] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4415842,cp2_bootup=15678,cp2_send=4415336\x0a
[ 4418.027202] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:04:41.561 boot time us:4418024738
[ 4418.027202] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:04:41.561 boot time us:4418024760
[ 4418.569575] (spc_manager) INFO, path: /blackbox, size: 3795.86MB -> 3708.14MB, rate: -175.4297MB/min
[ 4420.041031] (unrd) unrd_get_item Pid: 10803, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4420.887813] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4420882,15678\x0d\x0a
[ 4420.888498] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4420882,cp2_bootup=15678,cp2_send=4420375\x0a
[ 4425.780254] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10814
[ 4425.780826] (sh) handle_signal sig=1 current->comm=sh current->tgid=10814
[ 4425.781705] (sh) send_signal sig=1 current->comm=sh current->tgid=10814 t->comm=sh t->tgid=10814
[ 4425.927953] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4425922,15678\x0d\x0a
[ 4425.928668] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4425922,cp2_bootup=15678,cp2_send=4425415\x0a
[ 4426.229202] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10819
[ 4426.229831] (sh) handle_signal sig=1 current->comm=sh current->tgid=10819
[ 4426.230682] (sh) send_signal sig=1 current->comm=sh current->tgid=10819 t->comm=sh t->tgid=10819
[ 4426.758822] (sys_time_record)  Mon Nov 23 02:04:50 GMT 2026 GMT+1
[ 4428.030035] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:04:51.563 boot time us:4428027564
[ 4430.967976] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4430962,15678\x0d\x0a
[ 4430.968658] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4430962,cp2_bootup=15678,cp2_send=4430454\x0a
[ 4431.381617] (sh) send_signal sig=1 current->comm=sh current->tgid=10828 t->comm=sh t->tgid=10828
[ 4432.010632] (unrd) unrd_get_item Pid: 10831, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4436.017294] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4436012,15678\x0d\x0a
[ 4436.018100] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4436012,cp2_bootup=15678,cp2_send=4435503\x0a
[ 4438.032879] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:05:01.566 boot time us:4438030421
[ 4438.032944] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:05:01.566 boot time us:4438030506
[ 4441.047987] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4441042,15678\x0d\x0a
[ 4441.048657] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4441042,cp2_bootup=15678,cp2_send=4440533\x0a
[ 4443.898292] (sh) send_signal sig=1 current->comm=sh current->tgid=10849 t->comm=sh t->tgid=10849
[ 4443.981307] (unrd) unrd_get_item Pid: 10854, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4444.356894] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10858
[ 4444.357323] (sh) send_signal sig=1 current->comm=sh current->tgid=10858 t->comm=sh t->tgid=10858
[ 4445.112544] (logd.auditd) type=1400 audit(1795395849.189:174): avc: denied { open } for pid=10741 comm="cat" path="/proc/uptime" dev="proc" ino=4026532173 scontext=u:r:su:s0 tcontext=u:object_r:proc_uptime:s0 tclass=file permissive=1
[ 4445.112585] (logd.auditd) type=1400 audit(1795395908.629:175): avc: denied { search } for pid=3258 comm="sync_glass_loca" name="android_usb" dev="sysfs" ino=23585 scontext=u:r:su:s0 tcontext=u:object_r:sysfs_android_usb:s0 tclass=dir permissive=1
[ 4446.088195] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4446082,15678\x0d\x0a
[ 4446.088863] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4446082,cp2_bootup=15678,cp2_send=4445573\x0a
[ 4448.035459] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:05:11.569 boot time us:4448032956
[ 4448.588529] (spc_manager) INFO, path: /blackbox, size: 3708.14MB -> 3720.40MB, rate: 24.5078MB/min
[ 4451.127254] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4451122,15678\x0d\x0a
[ 4451.127927] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4451122,cp2_bootup=15678,cp2_send=4450612\x0a
[ 4455.953809] (unrd) unrd_get_item Pid: 10877, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4456.167569] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4456162,15678\x0d\x0a
[ 4456.168285] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4456162,cp2_bootup=15678,cp2_send=4455651\x0a
[ 4457.448916] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10882
[ 4457.449042] (sh) send_signal sig=1 current->comm=sh current->tgid=10882 t->comm=sh t->tgid=10882
[ 4457.776339] (logd.auditd) type=1400 audit(1795395908.629:175): avc: denied { search } for pid=3258 comm="sync_glass_loca" name="android_usb" dev="sysfs" ino=23585 scontext=u:r:su:s0 tcontext=u:object_r:sysfs_android_usb:s0 tclass=dir permissive=1
[ 4457.776887] (logd.auditd) type=1400 audit(1795395921.299:176): avc: denied { read } for pid=10888 comm="cat" name="meminfo" dev="proc" ino=4026532171 scontext=u:r:su:s0 tcontext=u:object_r:proc_meminfo:s0 tclass=file permissive=1
[ 4457.777817] (logd.auditd) type=1400 audit(1795395921.299:176): avc: denied { read } for pid=10888 comm="cat" name="meminfo" dev="proc" ino=4026532171 scontext=u:r:su:s0 tcontext=u:object_r:proc_meminfo:s0 tclass=file permissive=1
[ 4457.777855] (logd.auditd) type=1400 audit(1795395921.299:176): avc: denied { open } for pid=10888 comm="cat" path="/proc/meminfo" dev="proc" ino=4026532171 scontext=u:r:su:s0 tcontext=u:object_r:proc_meminfo:s0 tclass=file permissive=1
[ 4457.782627] (sh) send_signal sig=1 current->comm=sh current->tgid=10886 t->comm=sh t->tgid=10886
[ 4458.038445] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:05:21.572 boot time us:4458035969
[ 4461.208587] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4461202,15678\x0d\x0a
[ 4461.209241] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4461202,cp2_bootup=15678,cp2_send=4460692\x0a
[ 4461.362545] (dcs_server) unrd_set_item Pid: 998, Opt: Write, Size: 39944, Partition: /dev/block/by-name/cali_raw1 
[ 4461.383486] (sh) send_signal sig=1 current->comm=sh current->tgid=10894 t->comm=sh t->tgid=10894
[ 4461.383528] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10894
[ 4466.247496] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4466242,15678\x0d\x0a
[ 4466.248143] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4466242,cp2_bootup=15678,cp2_send=4465730\x0a
[ 4467.923813] (unrd) unrd_get_item Pid: 10905, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4468.041363] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:05:31.575 boot time us:4468038878
[ 4468.041363] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:05:31.575 boot time us:4468038879
[ 4471.286739] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4471282,15678\x0d\x0a
[ 4471.287390] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4471282,cp2_bootup=15678,cp2_send=4470769\x0a
[ 4472.544613] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=10913
[ 4472.544745] (sh) send_signal sig=1 current->comm=sh current->tgid=10913 t->comm=sh t->tgid=10913
[ 4473.194651] (sh) send_signal sig=1 current->comm=sh current->tgid=10918 t->comm=sh t->tgid=10918
[ 4476.336972] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4476332,15678\x0d\x0a
[ 4476.337652] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4476332,cp2_bootup=15678,cp2_send=4475819\x0a
[ 4478.044224] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:05:41.578 boot time us:4478041748
[ 4478.044224] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:05:41.578 boot time us:4478041748
[ 4478.607899] (spc_manager) INFO, path: /blackbox, size: 3720.40MB -> 3732.50MB, rate: 24.2031MB/min
[ 4479.905948] (unrd) unrd_get_item Pid: 10931, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4480.764671] (sh) send_signal sig=1 current->comm=sh current->tgid=10935 t->comm=sh t->tgid=10935
[ 4481.377704] (kworker/u8:3) WCN BASE: info: tx:at+loopcheck=4481372,15678\x0d\x0a
[ 4481.378369] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4481372,cp2_bootup=15678,cp2_send=4480859\x0a
[ 4486.411668] (kworker/u8:3) WCN BASE: info: tx:at+loopcheck=4486405,15678\x0d\x0a
[ 4486.412322] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4486405,cp2_bootup=15678,cp2_send=4485892\x0a
[ 4487.266111] (sys_time_record)  Mon Nov 23 02:05:50 GMT 2026 GMT+1
[ 4488.047110] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:05:51.581 boot time us:4488044641
[ 4488.047112] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:05:51.581 boot time us:4488044671
[ 4491.382314] (sh) send_signal sig=1 current->comm=sh current->tgid=10952 t->comm=sh t->tgid=10952
[ 4491.447019] (kworker/u8:3) WCN BASE: info: tx:at+loopcheck=4491442,15678\x0d\x0a
[ 4491.447711] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4491442,cp2_bootup=15678,cp2_send=4490927\x0a
[ 4491.875924] (unrd) unrd_get_item Pid: 10954, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4492.716396] (sh) send_signal sig=1 current->comm=sh current->tgid=10959 t->comm=sh t->tgid=10959
[ 4496.487808] (kworker/u8:3) WCN BASE: info: tx:at+loopcheck=4496482,15678\x0d\x0a
[ 4496.488466] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4496482,cp2_bootup=15678,cp2_send=4495967\x0a
[ 4498.049832] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:06:01.583 boot time us:4498047368
[ 4498.049833] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:06:01.583 boot time us:4498047371
[ 4501.527008] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4501522,15678\x0d\x0a
[ 4501.527716] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4501522,cp2_bootup=15678,cp2_send=4501006\x0a
[ 4503.847049] (unrd) unrd_get_item Pid: 10977, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4504.727202] (sh) send_signal sig=1 current->comm=sh current->tgid=10983 t->comm=sh t->tgid=10983
[ 4505.121608] (sh) send_signal sig=1 current->comm=sh current->tgid=10988 t->comm=sh t->tgid=10988
[ 4506.567814] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4506562,15678\x0d\x0a
[ 4506.568551] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4506562,cp2_bootup=15678,cp2_send=4506046\x0a
[ 4508.052686] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:06:11.586 boot time us:4508050216
[ 4508.626828] (spc_manager) INFO, path: /blackbox, size: 3732.50MB -> 3744.68MB, rate: 24.3516MB/min
[ 4511.607195] (kworker/u8:3) WCN BASE: info: tx:at+loopcheck=4511602,15678\x0d\x0a
[ 4511.607937] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4511602,cp2_bootup=15678,cp2_send=4511085\x0a
[ 4515.818460] (unrd) unrd_get_item Pid: 11007, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4515.825632] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=11005
[ 4515.825770] (sh) send_signal sig=1 current->comm=sh current->tgid=11005 t->comm=sh t->tgid=11005
[ 4516.088135] (sh) send_signal sig=1 current->comm=sh current->tgid=11014 t->comm=sh t->tgid=11014
[ 4516.656944] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4516652,15678\x0d\x0a
[ 4516.657683] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4516652,cp2_bootup=15678,cp2_send=4516134\x0a
[ 4518.055413] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:06:21.589 boot time us:4518052930
[ 4518.055414] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:06:21.589 boot time us:4518052936
[ 4521.386470] (sh) send_signal sig=1 current->comm=sh current->tgid=11023 t->comm=sh t->tgid=11023
[ 4521.687510] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4521682,15678\x0d\x0a
[ 4521.688190] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4521682,cp2_bootup=15678,cp2_send=4521164\x0a
[ 4523.377177] (dcs_server) unrd_set_item Pid: 998, Opt: Write, Size: 39944, Partition: /dev/block/by-name/cali_raw1 
[ 4526.737960] (kworker/u8:0) WCN BASE: info: tx:at+loopcheck=4526732,15678\x0d\x0a
[ 4526.738613] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4526732,cp2_bootup=15678,cp2_send=4526214\x0a
[ 4527.793468] (unrd) unrd_get_item Pid: 11033, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4528.058526] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:06:31.592 boot time us:4528056045
[ 4528.058526] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:06:31.592 boot time us:4528056045
[ 4531.768504] (kworker/u8:2) WCN BASE: info: tx:at+loopcheck=4531762,15678\x0d\x0a
[ 4531.769222] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4531762,cp2_bootup=15678,cp2_send=4531244\x0a
[ 4533.801596] (sh) send_signal sig=1 current->comm=sh current->tgid=11043 t->comm=sh t->tgid=11043
[ 4534.520266] (logd.auditd) type=1400 audit(1795395921.299:176): avc: denied { open } for pid=10888 comm="cat" path="/proc/meminfo" dev="proc" ino=4026532171 scontext=u:r:su:s0 tcontext=u:object_r:proc_meminfo:s0 tclass=file permissive=1
[ 4534.520306] (logd.auditd) type=1400 audit(1795395998.039:177): avc: denied { getattr } for pid=11048 comm="sh" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.521120] (logd.auditd) type=1400 audit(1795395998.039:177): avc: denied { getattr } for pid=11048 comm="sh" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.521158] (logd.auditd) type=1400 audit(1795395998.039:178): avc: denied { execute } for pid=11048 comm="sh" name="logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.521354] (logd.auditd) type=1400 audit(1795395998.039:178): avc: denied { execute } for pid=11048 comm="sh" name="logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.521379] (logd.auditd) type=1400 audit(1795395998.039:179): avc: denied { read open } for pid=11050 comm="sh" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.521872] (logd.auditd) type=1400 audit(1795395998.039:179): avc: denied { read open } for pid=11050 comm="sh" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.521893] (logd.auditd) type=1400 audit(1795395998.039:179): avc: denied { execute_no_trans } for pid=11050 comm="sh" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.525912] (logd.auditd) type=1400 audit(1795395998.039:179): avc: denied { execute_no_trans } for pid=11050 comm="sh" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.525950] (logd.auditd) type=1400 audit(1795395998.039:179): avc: denied { map } for pid=11050 comm="logcat" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.550639] (logd.auditd) type=1400 audit(1795395998.039:179): avc: denied { map } for pid=11050 comm="logcat" path="/system/bin/logcat" dev="dm-0" ino=1255 scontext=u:r:su:s0 tcontext=u:object_r:logcat_exec:s0 tclass=file permissive=1
[ 4534.550675] (logd.auditd) type=1400 audit(1795395998.069:180): avc: denied { write } for pid=11050 comm="logcat" name="logdr" dev="devtmpfs" ino=2466 scontext=u:r:su:s0 tcontext=u:object_r:logdr_socket:s0 tclass=sock_file permissive=1
[ 4534.552578] (logd.auditd) type=1400 audit(1795395998.069:180): avc: denied { write } for pid=11050 comm="logcat" name="logdr" dev="devtmpfs" ino=2466 scontext=u:r:su:s0 tcontext=u:object_r:logdr_socket:s0 tclass=sock_file permissive=1
[ 4534.552620] (logd.auditd) type=1400 audit(1795395998.069:180): avc: denied { connectto } for pid=11050 comm="logcat" path="/dev/socket/logdr" scontext=u:r:su:s0 tcontext=u:r:logd:s0 tclass=unix_stream_socket permissive=1
[ 4534.552780] (logd.reader) logd: logdr: UID=0 GID=0 PID=11050 n tail=300 logMask=19 pid=0 start=0ns timeout=0ns
[ 4534.590350] (logcat) failed to read android log socket: 0, No such file or directory
[ 4534.590375] (logcat) failed to read android log, ret: -11
[ 4534.598141] (sh) send_signal sig=1 current->comm=sh current->tgid=11048 t->comm=sh t->tgid=11048
[ 4535.157930] (sync_glass_loca) audit: audit_lost=105 audit_rate_limit=5 audit_backlog_limit=64
[ 4535.157942] (sync_glass_loca) audit: rate limit exceeded
[ 4535.158027] (logd.auditd) type=1400 audit(1795395998.069:180): avc: denied { connectto } for pid=11050 comm="logcat" path="/dev/socket/logdr" scontext=u:r:su:s0 tcontext=u:r:logd:s0 tclass=unix_stream_socket permissive=1
[ 4535.158063] (logd.auditd) type=1400 audit(1795395998.679:181): avc: denied { read } for pid=3258 comm="sync_glass_loca" name="state" dev="sysfs" ino=28410 scontext=u:r:su:s0 tcontext=u:object_r:sysfs_android_usb:s0 tclass=file permissive=1
[ 4536.808452] (kworker/u8:6) WCN BASE: info: tx:at+loopcheck=4536802,15678\x0d\x0a
[ 4536.809100] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4536802,cp2_bootup=15678,cp2_send=4536284\x0a
[ 4538.061388] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:06:41.595 boot time us:4538058906
[ 4538.646006] (spc_manager) INFO, path: /blackbox, size: 3744.68MB -> 3756.82MB, rate: 24.2812MB/min
[ 4539.765167] (unrd) unrd_get_item Pid: 11060, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4541.848443] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4541842,15678\x0d\x0a
[ 4541.849105] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4541842,cp2_bootup=15678,cp2_send=4541323\x0a
[ 4546.888487] (kworker/u8:6) WCN BASE: info: tx:at+loopcheck=4546882,15678\x0d\x0a
[ 4546.889165] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4546882,cp2_bootup=15678,cp2_send=4546363\x0a
[ 4547.792090] (sys_time_record)  Mon Nov 23 02:06:51 GMT 2026 GMT+1
[ 4548.064309] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:06:51.598 boot time us:4548061853
[ 4551.387832] (sh) send_signal sig=1 current->comm=sh current->tgid=11078 t->comm=sh t->tgid=11078
[ 4551.738624] (unrd) unrd_get_item Pid: 11080, Opt: Read, Size: 41, Partition: /dev/block/by-name/env 
[ 4551.927424] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4551922,15678\x0d\x0a
[ 4551.928087] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4551922,cp2_bootup=15678,cp2_send=4551401\x0a
[ 4556.970411] (kworker/u8:1) WCN BASE: info: tx:at+loopcheck=4556964,15678\x0d\x0a
[ 4556.971102] (sdiohal_rx_thre) WCN BASE: info: rx:loopcheck_ack:ap_send=4556964,cp2_bootup=15678,cp2_send=4556443\x0a
[ 4558.067061] (dspf_1_monitor) [DSPF]: dsp:1 monitor thread current date:2026-11-23 02:07:01.600 boot time us:4558064595
[ 4558.067066] (dspf_0_monitor) [DSPF]: dsp:0 monitor thread current date:2026-11-23 02:07:01.600 boot time us:4558064594
[ 4559.969753] (adbd) send_signal sig=1 current->comm=adbd current->tgid=2430 t->comm=sh t->tgid=11095
[ 4559.969855] (sh) send_signal sig=1 current->comm=sh current->tgid=11095 t->comm=sh t->tgid=11095
```

## Display

```shell
cat /proc/display/feature

VOP 0 - vop_lcdc0:
        work mode: blending
        out caps: clip, bkg
        out  src: offline
        out port: BT, BT, BT, BT
        plane 4 - video:
                fmt bitmap: 0x0000010506600104
                plane caps: crop, blend, dimming, peaking, zebra, hist, fcolor, proc_area
                plane  src: offline
        plane 5 - gfx0:
                fmt bitmap: 0x0000010506600104
                plane caps: crop, blend, dimming
                plane  src: offline
        plane 6 - gfx1:
                fmt bitmap: 0x0000010506600104
                plane caps: crop, blend, dimming
                plane  src: offline
        plane 7 - wrb:
                fmt bitmap: 0x0000000002000034
                plane caps: 
                plane  src: offline
VOP 1 - vop_lcdc1:
        work mode: blending
        out caps: bkg
        out  src: offline
        out port: BT, BT
        plane 4 - video:
                fmt bitmap: 0x0000010506600104
                plane caps: crop, blend, dimming, peaking, zebra, hist, fcolor, proc_area
                plane  src: offline
        plane 5 - gfx0:
                fmt bitmap: 0x0000010506600104
                plane caps: crop, blend, dimming
                plane  src: offline
```

```shell
cat /proc/display/plane
VOP 0 - vop_lcdc0:
        plane 4 - video:
                status   : user disabled, actually disabled, src: offline
                frame    : no input frame
                crop     : disabled
                blending : pos: 0-0, order: 0, alpha: 0, alg: 0
        plane 5 - gfx0:
                status   : user enabled, actually enabled, src: offline
                frame    : res: 1920x1080, fmt: argb8888, idx: 0, phy: 0x21b16000
                           frame cnt: 14, in queue: 0, out queue: 1
                           frame total drop: 0, repeat: 2233, total repeat: 126177
                           frame cfg_frame_by_sequence: 0, cfg_frame_delay_cnt: 0
                crop     : disabled
                blending : pos: 0-0, order: 2, alpha: pixel, alg: 0
        plane 6 - gfx1:
                status   : user enabled, actually enabled, src: offline
                frame    : res: 1920x1080, fmt: argb8888, idx: 0, phy: 0x2ee17000
                           frame cnt: 1, in queue: 0, out queue: 1
                           frame total drop: 0, repeat: 127876, total repeat: 127876
                           frame cfg_frame_by_sequence: 0, cfg_frame_delay_cnt: 0
                crop     : disabled
                blending : pos: 0-0, order: 1, alpha: pixel, alg: 0
        plane 7 - wrb:
                status   : user enabled, actually enabled, src: offline
                frame    : res: 1920x1080, fmt: rgb888-i32, idx: 2, phy: 0x2925d000
                           frame cnt: 64018, in queue: 0, out queue: 1
                           frame total drop: 0, repeat: 0, total repeat: 64025
                           frame cfg_frame_by_sequence: 0, cfg_frame_delay_cnt: 0
                wrb ref  : none-0
VOP 1 - vop_lcdc1:
        plane 4 - video:
                status   : user disabled, actually disabled, src: offline
                frame    : no input frame
                crop     : disabled
                blending : pos: 0-0, order: 0, alpha: 0, alg: 0
        plane 5 - gfx0:
                status   : user disabled, actually disabled, src: offline
                frame    : no input frame
                crop     : disabled
                blending : disabled
```

```shell
cat /proc/display/timing
               CLK         FPS  HAC  HFP  HBP  HSW  VAC  VFP  VBP  VSW  RES TYPE
VOP 0:   237600000  100.079187 1920  120   51   44 1080   16   14    2 2012 active[native]
         237600000  100.079187 1920  120   51   44 1080   16   14    2 2012 boe_049_re[native]
VOP 1:    62526000   59.912382 1280   64   12   10  720   28   14    2    0 active
          62530000   59.916215 1280   64   12   10  720   28   14    2    4 probe-0[native]
```
