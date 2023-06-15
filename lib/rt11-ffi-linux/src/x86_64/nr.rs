// This code is generated.
pub const READ: u64 = 0;
pub const WRITE: u64 = 1;
pub const OPEN: u64 = 2;
pub const CLOSE: u64 = 3;
pub const STAT: u64 = 4;
pub const FSTAT: u64 = 5;
pub const LSTAT: u64 = 6;
pub const POLL: u64 = 7;
pub const LSEEK: u64 = 8;
pub const MMAP: u64 = 9;
pub const MPROTECT: u64 = 10;
pub const MUNMAP: u64 = 11;
pub const BRK: u64 = 12;
pub const RT_SIGACTION: u64 = 13;
pub const RT_SIGPROCMASK: u64 = 14;
pub const RT_SIGRETURN: u64 = 15;
pub const IOCTL: u64 = 16;
pub const PREAD64: u64 = 17;
pub const PWRITE64: u64 = 18;
pub const READV: u64 = 19;
pub const WRITEV: u64 = 20;
pub const ACCESS: u64 = 21;
pub const PIPE: u64 = 22;
pub const SELECT: u64 = 23;
pub const SCHED_YIELD: u64 = 24;
pub const MREMAP: u64 = 25;
pub const MSYNC: u64 = 26;
pub const MINCORE: u64 = 27;
pub const MADVISE: u64 = 28;
pub const SHMGET: u64 = 29;
pub const SHMAT: u64 = 30;
pub const SHMCTL: u64 = 31;
pub const DUP: u64 = 32;
pub const DUP2: u64 = 33;
pub const PAUSE: u64 = 34;
pub const NANOSLEEP: u64 = 35;
pub const GETITIMER: u64 = 36;
pub const ALARM: u64 = 37;
pub const SETITIMER: u64 = 38;
pub const GETPID: u64 = 39;
pub const SENDFILE: u64 = 40;
pub const SOCKET: u64 = 41;
pub const CONNECT: u64 = 42;
pub const ACCEPT: u64 = 43;
pub const SENDTO: u64 = 44;
pub const RECVFROM: u64 = 45;
pub const SENDMSG: u64 = 46;
pub const RECVMSG: u64 = 47;
pub const SHUTDOWN: u64 = 48;
pub const BIND: u64 = 49;
pub const LISTEN: u64 = 50;
pub const GETSOCKNAME: u64 = 51;
pub const GETPEERNAME: u64 = 52;
pub const SOCKETPAIR: u64 = 53;
pub const SETSOCKOPT: u64 = 54;
pub const GETSOCKOPT: u64 = 55;
pub const CLONE: u64 = 56;
pub const FORK: u64 = 57;
pub const VFORK: u64 = 58;
pub const EXECVE: u64 = 59;
pub const EXIT: u64 = 60;
pub const WAIT4: u64 = 61;
pub const KILL: u64 = 62;
pub const UNAME: u64 = 63;
pub const SEMGET: u64 = 64;
pub const SEMOP: u64 = 65;
pub const SEMCTL: u64 = 66;
pub const SHMDT: u64 = 67;
pub const MSGGET: u64 = 68;
pub const MSGSND: u64 = 69;
pub const MSGRCV: u64 = 70;
pub const MSGCTL: u64 = 71;
pub const FCNTL: u64 = 72;
pub const FLOCK: u64 = 73;
pub const FSYNC: u64 = 74;
pub const FDATASYNC: u64 = 75;
pub const TRUNCATE: u64 = 76;
pub const FTRUNCATE: u64 = 77;
pub const GETDENTS: u64 = 78;
pub const GETCWD: u64 = 79;
pub const CHDIR: u64 = 80;
pub const FCHDIR: u64 = 81;
pub const RENAME: u64 = 82;
pub const MKDIR: u64 = 83;
pub const RMDIR: u64 = 84;
pub const CREAT: u64 = 85;
pub const LINK: u64 = 86;
pub const UNLINK: u64 = 87;
pub const SYMLINK: u64 = 88;
pub const READLINK: u64 = 89;
pub const CHMOD: u64 = 90;
pub const FCHMOD: u64 = 91;
pub const CHOWN: u64 = 92;
pub const FCHOWN: u64 = 93;
pub const LCHOWN: u64 = 94;
pub const UMASK: u64 = 95;
pub const GETTIMEOFDAY: u64 = 96;
pub const GETRLIMIT: u64 = 97;
pub const GETRUSAGE: u64 = 98;
pub const SYSINFO: u64 = 99;
pub const TIMES: u64 = 100;
pub const PTRACE: u64 = 101;
pub const GETUID: u64 = 102;
pub const SYSLOG: u64 = 103;
pub const GETGID: u64 = 104;
pub const SETUID: u64 = 105;
pub const SETGID: u64 = 106;
pub const GETEUID: u64 = 107;
pub const GETEGID: u64 = 108;
pub const SETPGID: u64 = 109;
pub const GETPPID: u64 = 110;
pub const GETPGRP: u64 = 111;
pub const SETSID: u64 = 112;
pub const SETREUID: u64 = 113;
pub const SETREGID: u64 = 114;
pub const GETGROUPS: u64 = 115;
pub const SETGROUPS: u64 = 116;
pub const SETRESUID: u64 = 117;
pub const GETRESUID: u64 = 118;
pub const SETRESGID: u64 = 119;
pub const GETRESGID: u64 = 120;
pub const GETPGID: u64 = 121;
pub const SETFSUID: u64 = 122;
pub const SETFSGID: u64 = 123;
pub const GETSID: u64 = 124;
pub const CAPGET: u64 = 125;
pub const CAPSET: u64 = 126;
pub const RT_SIGPENDING: u64 = 127;
pub const RT_SIGTIMEDWAIT: u64 = 128;
pub const RT_SIGQUEUEINFO: u64 = 129;
pub const RT_SIGSUSPEND: u64 = 130;
pub const SIGALTSTACK: u64 = 131;
pub const UTIME: u64 = 132;
pub const MKNOD: u64 = 133;
pub const USELIB: u64 = 134;
pub const PERSONALITY: u64 = 135;
pub const USTAT: u64 = 136;
pub const STATFS: u64 = 137;
pub const FSTATFS: u64 = 138;
pub const SYSFS: u64 = 139;
pub const GETPRIORITY: u64 = 140;
pub const SETPRIORITY: u64 = 141;
pub const SCHED_SETPARAM: u64 = 142;
pub const SCHED_GETPARAM: u64 = 143;
pub const SCHED_SETSCHEDULER: u64 = 144;
pub const SCHED_GETSCHEDULER: u64 = 145;
pub const SCHED_GET_PRIORITY_MAX: u64 = 146;
pub const SCHED_GET_PRIORITY_MIN: u64 = 147;
pub const SCHED_RR_GET_INTERVAL: u64 = 148;
pub const MLOCK: u64 = 149;
pub const MUNLOCK: u64 = 150;
pub const MLOCKALL: u64 = 151;
pub const MUNLOCKALL: u64 = 152;
pub const VHANGUP: u64 = 153;
pub const MODIFY_LDT: u64 = 154;
pub const PIVOT_ROOT: u64 = 155;
pub const _SYSCTL: u64 = 156;
pub const PRCTL: u64 = 157;
pub const ARCH_PRCTL: u64 = 158;
pub const ADJTIMEX: u64 = 159;
pub const SETRLIMIT: u64 = 160;
pub const CHROOT: u64 = 161;
pub const SYNC: u64 = 162;
pub const ACCT: u64 = 163;
pub const SETTIMEOFDAY: u64 = 164;
pub const MOUNT: u64 = 165;
pub const UMOUNT2: u64 = 166;
pub const SWAPON: u64 = 167;
pub const SWAPOFF: u64 = 168;
pub const REBOOT: u64 = 169;
pub const SETHOSTNAME: u64 = 170;
pub const SETDOMAINNAME: u64 = 171;
pub const IOPL: u64 = 172;
pub const IOPERM: u64 = 173;
pub const CREATE_MODULE: u64 = 174;
pub const INIT_MODULE: u64 = 175;
pub const DELETE_MODULE: u64 = 176;
pub const GET_KERNEL_SYMS: u64 = 177;
pub const QUERY_MODULE: u64 = 178;
pub const QUOTACTL: u64 = 179;
pub const NFSSERVCTL: u64 = 180;
pub const GETPMSG: u64 = 181;
pub const PUTPMSG: u64 = 182;
pub const AFS_SYSCALL: u64 = 183;
pub const TUXCALL: u64 = 184;
pub const SECURITY: u64 = 185;
pub const GETTID: u64 = 186;
pub const READAHEAD: u64 = 187;
pub const SETXATTR: u64 = 188;
pub const LSETXATTR: u64 = 189;
pub const FSETXATTR: u64 = 190;
pub const GETXATTR: u64 = 191;
pub const LGETXATTR: u64 = 192;
pub const FGETXATTR: u64 = 193;
pub const LISTXATTR: u64 = 194;
pub const LLISTXATTR: u64 = 195;
pub const FLISTXATTR: u64 = 196;
pub const REMOVEXATTR: u64 = 197;
pub const LREMOVEXATTR: u64 = 198;
pub const FREMOVEXATTR: u64 = 199;
pub const TKILL: u64 = 200;
pub const TIME: u64 = 201;
pub const FUTEX: u64 = 202;
pub const SCHED_SETAFFINITY: u64 = 203;
pub const SCHED_GETAFFINITY: u64 = 204;
pub const SET_THREAD_AREA: u64 = 205;
pub const IO_SETUP: u64 = 206;
pub const IO_DESTROY: u64 = 207;
pub const IO_GETEVENTS: u64 = 208;
pub const IO_SUBMIT: u64 = 209;
pub const IO_CANCEL: u64 = 210;
pub const GET_THREAD_AREA: u64 = 211;
pub const LOOKUP_DCOOKIE: u64 = 212;
pub const EPOLL_CREATE: u64 = 213;
pub const EPOLL_CTL_OLD: u64 = 214;
pub const EPOLL_WAIT_OLD: u64 = 215;
pub const REMAP_FILE_PAGES: u64 = 216;
pub const GETDENTS64: u64 = 217;
pub const SET_TID_ADDRESS: u64 = 218;
pub const RESTART_SYSCALL: u64 = 219;
pub const SEMTIMEDOP: u64 = 220;
pub const FADVISE64: u64 = 221;
pub const TIMER_CREATE: u64 = 222;
pub const TIMER_SETTIME: u64 = 223;
pub const TIMER_GETTIME: u64 = 224;
pub const TIMER_GETOVERRUN: u64 = 225;
pub const TIMER_DELETE: u64 = 226;
pub const CLOCK_SETTIME: u64 = 227;
pub const CLOCK_GETTIME: u64 = 228;
pub const CLOCK_GETRES: u64 = 229;
pub const CLOCK_NANOSLEEP: u64 = 230;
pub const EXIT_GROUP: u64 = 231;
pub const EPOLL_WAIT: u64 = 232;
pub const EPOLL_CTL: u64 = 233;
pub const TGKILL: u64 = 234;
pub const UTIMES: u64 = 235;
pub const VSERVER: u64 = 236;
pub const MBIND: u64 = 237;
pub const SET_MEMPOLICY: u64 = 238;
pub const GET_MEMPOLICY: u64 = 239;
pub const MQ_OPEN: u64 = 240;
pub const MQ_UNLINK: u64 = 241;
pub const MQ_TIMEDSEND: u64 = 242;
pub const MQ_TIMEDRECEIVE: u64 = 243;
pub const MQ_NOTIFY: u64 = 244;
pub const MQ_GETSETATTR: u64 = 245;
pub const KEXEC_LOAD: u64 = 246;
pub const WAITID: u64 = 247;
pub const ADD_KEY: u64 = 248;
pub const REQUEST_KEY: u64 = 249;
pub const KEYCTL: u64 = 250;
pub const IOPRIO_SET: u64 = 251;
pub const IOPRIO_GET: u64 = 252;
pub const INOTIFY_INIT: u64 = 253;
pub const INOTIFY_ADD_WATCH: u64 = 254;
pub const INOTIFY_RM_WATCH: u64 = 255;
pub const MIGRATE_PAGES: u64 = 256;
pub const OPENAT: u64 = 257;
pub const MKDIRAT: u64 = 258;
pub const MKNODAT: u64 = 259;
pub const FCHOWNAT: u64 = 260;
pub const FUTIMESAT: u64 = 261;
pub const NEWFSTATAT: u64 = 262;
pub const UNLINKAT: u64 = 263;
pub const RENAMEAT: u64 = 264;
pub const LINKAT: u64 = 265;
pub const SYMLINKAT: u64 = 266;
pub const READLINKAT: u64 = 267;
pub const FCHMODAT: u64 = 268;
pub const FACCESSAT: u64 = 269;
pub const PSELECT6: u64 = 270;
pub const PPOLL: u64 = 271;
pub const UNSHARE: u64 = 272;
pub const SET_ROBUST_LIST: u64 = 273;
pub const GET_ROBUST_LIST: u64 = 274;
pub const SPLICE: u64 = 275;
pub const TEE: u64 = 276;
pub const SYNC_FILE_RANGE: u64 = 277;
pub const VMSPLICE: u64 = 278;
pub const MOVE_PAGES: u64 = 279;
pub const UTIMENSAT: u64 = 280;
pub const EPOLL_PWAIT: u64 = 281;
pub const SIGNALFD: u64 = 282;
pub const TIMERFD_CREATE: u64 = 283;
pub const EVENTFD: u64 = 284;
pub const FALLOCATE: u64 = 285;
pub const TIMERFD_SETTIME: u64 = 286;
pub const TIMERFD_GETTIME: u64 = 287;
pub const ACCEPT4: u64 = 288;
pub const SIGNALFD4: u64 = 289;
pub const EVENTFD2: u64 = 290;
pub const EPOLL_CREATE1: u64 = 291;
pub const DUP3: u64 = 292;
pub const PIPE2: u64 = 293;
pub const INOTIFY_INIT1: u64 = 294;
pub const PREADV: u64 = 295;
pub const PWRITEV: u64 = 296;
pub const RT_TGSIGQUEUEINFO: u64 = 297;
pub const PERF_EVENT_OPEN: u64 = 298;
pub const RECVMMSG: u64 = 299;
pub const FANOTIFY_INIT: u64 = 300;
pub const FANOTIFY_MARK: u64 = 301;
pub const PRLIMIT64: u64 = 302;
pub const NAME_TO_HANDLE_AT: u64 = 303;
pub const OPEN_BY_HANDLE_AT: u64 = 304;
pub const CLOCK_ADJTIME: u64 = 305;
pub const SYNCFS: u64 = 306;
pub const SENDMMSG: u64 = 307;
pub const SETNS: u64 = 308;
pub const GETCPU: u64 = 309;
pub const PROCESS_VM_READV: u64 = 310;
pub const PROCESS_VM_WRITEV: u64 = 311;
pub const KCMP: u64 = 312;
pub const FINIT_MODULE: u64 = 313;
pub const SCHED_SETATTR: u64 = 314;
pub const SCHED_GETATTR: u64 = 315;
pub const RENAMEAT2: u64 = 316;
pub const SECCOMP: u64 = 317;
pub const GETRANDOM: u64 = 318;
pub const MEMFD_CREATE: u64 = 319;
pub const KEXEC_FILE_LOAD: u64 = 320;
pub const BPF: u64 = 321;
pub const EXECVEAT: u64 = 322;
pub const USERFAULTFD: u64 = 323;
pub const MEMBARRIER: u64 = 324;
pub const MLOCK2: u64 = 325;
pub const COPY_FILE_RANGE: u64 = 326;
pub const PREADV2: u64 = 327;
pub const PWRITEV2: u64 = 328;
pub const PKEY_MPROTECT: u64 = 329;
pub const PKEY_ALLOC: u64 = 330;
pub const PKEY_FREE: u64 = 331;
pub const STATX: u64 = 332;
pub const IO_PGETEVENTS: u64 = 333;
pub const RSEQ: u64 = 334;
pub const PIDFD_SEND_SIGNAL: u64 = 424;
pub const IO_URING_SETUP: u64 = 425;
pub const IO_URING_ENTER: u64 = 426;
pub const IO_URING_REGISTER: u64 = 427;
pub const OPEN_TREE: u64 = 428;
pub const MOVE_MOUNT: u64 = 429;
pub const FSOPEN: u64 = 430;
pub const FSCONFIG: u64 = 431;
pub const FSMOUNT: u64 = 432;
pub const FSPICK: u64 = 433;
pub const PIDFD_OPEN: u64 = 434;
pub const CLONE3: u64 = 435;
pub const CLOSE_RANGE: u64 = 436;
pub const OPENAT2: u64 = 437;
pub const PIDFD_GETFD: u64 = 438;
pub const FACCESSAT2: u64 = 439;
pub const PROCESS_MADVISE: u64 = 440;
pub const EPOLL_PWAIT2: u64 = 441;
pub const MOUNT_SETATTR: u64 = 442;
pub const QUOTACTL_FD: u64 = 443;
pub const LANDLOCK_CREATE_RULESET: u64 = 444;
pub const LANDLOCK_ADD_RULE: u64 = 445;
pub const LANDLOCK_RESTRICT_SELF: u64 = 446;
pub const MEMFD_SECRET: u64 = 447;
pub const PROCESS_MRELEASE: u64 = 448;
pub const FUTEX_WAITV: u64 = 449;
pub const SET_MEMPOLICY_HOME_NODE: u64 = 450;
