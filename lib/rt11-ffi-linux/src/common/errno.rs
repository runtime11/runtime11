//! Error Codes
//!
//! Linux follows the UNIX tradition in using integer error codes across all
//! its interfaces. These are untyped and can be returned by any code in the
//! kernel. Some have special meaning you can rely upon, yet this is not
//! enforced by the kernel in any way.

// Base error codes
pub const EPERM: u16 = 1;
pub const ENOENT: u16 = 2;
pub const ESRCH: u16 = 3;
pub const EINTR: u16 = 4;
pub const EIO: u16 = 5;
pub const ENXIO: u16 = 6;
pub const E2BIG: u16 = 7;
pub const ENOEXEC: u16 = 8;
pub const EBADF: u16 = 9;
pub const ECHILD: u16 = 10;
pub const EAGAIN: u16 = 11;
pub const ENOMEM: u16 = 12;
pub const EACCES: u16 = 13;
pub const EFAULT: u16 = 14;
pub const ENOTBLK: u16 = 15;
pub const EBUSY: u16 = 16;
pub const EEXIST: u16 = 17;
pub const EXDEV: u16 = 18;
pub const ENODEV: u16 = 19;
pub const ENOTDIR: u16 = 20;
pub const EISDIR: u16 = 21;
pub const EINVAL: u16 = 22;
pub const ENFILE: u16 = 23;
pub const EMFILE: u16 = 24;
pub const ENOTTY: u16 = 25;
pub const ETXTBSY: u16 = 26;
pub const EFBIG: u16 = 27;
pub const ENOSPC: u16 = 28;
pub const ESPIPE: u16 = 29;
pub const EROFS: u16 = 30;
pub const EMLINK: u16 = 31;
pub const EPIPE: u16 = 32;
pub const EDOM: u16 = 33;
pub const ERANGE: u16 = 34;

// Extended error codes
pub const EDEADLK: u16 = 35;
pub const ENAMETOOLONG: u16 = 36;
pub const ENOLCK: u16 = 37;
pub const ENOSYS: u16 = 38;
pub const ENOTEMPTY: u16 = 39;
pub const ELOOP: u16 = 40;
// 41 is unused (EWOULDBLOCK on UNIX).
pub const ENOMSG: u16 = 42;
pub const EIDRM: u16 = 43;
pub const ECHRNG: u16 = 44;
pub const EL2NSYNC: u16 = 45;
pub const EL3HLT: u16 = 46;
pub const EL3RST: u16 = 47;
pub const ELNRNG: u16 = 48;
pub const EUNATCH: u16 = 49;
pub const ENOCSI: u16 = 50;
pub const EL2HLT: u16 = 51;
pub const EBADE: u16 = 52;
pub const EBADR: u16 = 53;
pub const EXFULL: u16 = 54;
pub const ENOANO: u16 = 55;
pub const EBADRQC: u16 = 56;
pub const EBADSLT: u16 = 57;
// 58 is unused (EDEADLOCK on UNIX).
pub const EBFONT: u16 = 59;
pub const ENOSTR: u16 = 60;
pub const ENODATA: u16 = 61;
pub const ETIME: u16 = 62;
pub const ENOSR: u16 = 63;
pub const ENONET: u16 = 64;
pub const ENOPKG: u16 = 65;
pub const EREMOTE: u16 = 66;
pub const ENOLINK: u16 = 67;
pub const EADV: u16 = 68;
pub const ESRMNT: u16 = 69;
pub const ECOMM: u16 = 70;
pub const EPROTO: u16 = 71;
pub const EMULTIHOP: u16 = 72;
pub const EDOTDOT: u16 = 73;
pub const EBADMSG: u16 = 74;
pub const EOVERFLOW: u16 = 75;
pub const ENOTUNIQ: u16 = 76;
pub const EBADFD: u16 = 77;
pub const EREMCHG: u16 = 78;
pub const ELIBACC: u16 = 79;
pub const ELIBBAD: u16 = 80;
pub const ELIBSCN: u16 = 81;
pub const ELIBMAX: u16 = 82;
pub const ELIBEXEC: u16 = 83;
pub const EILSEQ: u16 = 84;
pub const ERESTART: u16 = 85;
pub const ESTRPIPE: u16 = 86;
pub const EUSERS: u16 = 87;
pub const ENOTSOCK: u16 = 88;
pub const EDESTADDRREQ: u16 = 89;
pub const EMSGSIZE: u16 = 90;
pub const EPROTOTYPE: u16 = 91;
pub const ENOPROTOOPT: u16 = 92;
pub const EPROTONOSUPPORT: u16 = 93;
pub const ESOCKTNOSUPPORT: u16 = 94;
pub const EOPNOTSUPP: u16 = 95;
pub const EPFNOSUPPORT: u16 = 96;
pub const EAFNOSUPPORT: u16 = 97;
pub const EADDRINUSE: u16 = 98;
pub const EADDRNOTAVAIL: u16 = 99;
pub const ENETDOWN: u16 = 100;
pub const ENETUNREACH: u16 = 101;
pub const ENETRESET: u16 = 102;
pub const ECONNABORTED: u16 = 103;
pub const ECONNRESET: u16 = 104;
pub const ENOBUFS: u16 = 105;
pub const EISCONN: u16 = 106;
pub const ENOTCONN: u16 = 107;
pub const ESHUTDOWN: u16 = 108;
pub const ETOOMANYREFS: u16 = 109;
pub const ETIMEDOUT: u16 = 110;
pub const ECONNREFUSED: u16 = 111;
pub const EHOSTDOWN: u16 = 112;
pub const EHOSTUNREACH: u16 = 113;
pub const EALREADY: u16 = 114;
pub const EINPROGRESS: u16 = 115;
pub const ESTALE: u16 = 116;
pub const EUCLEAN: u16 = 117;
pub const ENOTNAM: u16 = 118;
pub const ENAVAIL: u16 = 119;
pub const EISNAM: u16 = 120;
pub const EREMOTEIO: u16 = 121;
pub const EDQUOT: u16 = 122;
pub const ENOMEDIUM: u16 = 123;
pub const EMEDIUMTYPE: u16 = 124;
pub const ECANCELED: u16 = 125;
pub const ENOKEY: u16 = 126;
pub const EKEYEXPIRED: u16 = 127;
pub const EKEYREVOKED: u16 = 128;
pub const EKEYREJECTED: u16 = 129;
pub const EOWNERDEAD: u16 = 130;
pub const ENOTRECOVERABLE: u16 = 131;
pub const ERFKILL: u16 = 132;
pub const EHWPOISON: u16 = 133;

// NFS codes that have found their use in other drivers as well.
pub const EBADHANDLE: u16 = 521;
pub const ENOTSYNC: u16 = 522;
pub const EBADCOOKIE: u16 = 523;
pub const ENOTSUPP: u16 = 524;
pub const ETOOSMALL: u16 = 525;
pub const ESERVERFAULT: u16 = 526;
pub const EBADTYPE: u16 = 527;
pub const EJUKEBOX: u16 = 528;
pub const EIOCBQUEUED: u16 = 529;
pub const ERECALLCONFLICT: u16 = 530;
pub const ENOGRACE: u16 = 531;

// Aliases to the canonical linux codes.
pub const EWOULDBLOCK: u16 = 11; // EAGAIN
pub const EDEADLOCK: u16 = 35; // EDEADLCK
