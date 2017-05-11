```
".", 1)                         = 1
write(2, ".", 1)                        = 1
read(0, "/", 1)                         = 1
write(2, "/", 1)                        = 1
read(0, "a", 1)                         = 1
write(2, "a", 1)                        = 1
read(0, ".", 1)                         = 1
write(2, ".", 1)                        = 1
read(0, "o", 1)                         = 1
write(2, "o", 1)                        = 1
read(0, "u", 1)                         = 1
write(2, "u", 1)                        = 1
read(0, "t", 1)                         = 1
write(2, "t", 1)                        = 1
read(0, "\r", 1)                        = 1
write(2, "\n", 1)                       = 1
ioctl(0, TCGETS, {B38400 opost isig -icanon -echo ...}) = 0
ioctl(0, SNDCTL_TMR_STOP or TCSETSW, {B38400 opost isig icanon echo ...}) = 0
ioctl(0, TCGETS, {B38400 opost isig icanon echo ...}) = 0
rt_sigaction(SIGINT, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTERM, {0x45f230, [], SA_RESTORER|SA_RESTART, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGHUP, {0x45f960, [HUP INT ILL TRAP ABRT BUS FPE USR1 SEGV USR2 PIPE ALRM TERM XCPU XFSZ VTALRM SYS], SA_RESTORER, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGALRM, {0x45f960, [HUP INT ILL TRAP ABRT BUS FPE USR1 SEGV USR2 PIPE ALRM TERM XCPU XFSZ VTALRM SYS], SA_RESTORER, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGWINCH, {0x45f220, [], SA_RESTORER, 0x7fa5496b64b0}, {0x4ac3f0, [], SA_RESTORER|SA_RESTART, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGINT, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigprocmask(SIG_BLOCK, [INT CHLD], [], 8) = 0
pipe([3, 4])                            = 0
clone(child_stack=0, flags=CLONE_CHILD_CLEARTID|CLONE_CHILD_SETTID|SIGCHLD, child_tidptr=0x7fa54a07e9d0) = 29657
setpgid(29657, 29657)                   = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
rt_sigprocmask(SIG_BLOCK, [CHLD], [], 8) = 0
close(3)                                = 0
close(4)                                = 0
ioctl(255, TIOCGPGRP, [29571])          = 0
rt_sigprocmask(SIG_BLOCK, [CHLD TSTP TTIN TTOU], [CHLD], 8) = 0
ioctl(255, TIOCSPGRP, [29657])          = 0
rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
rt_sigprocmask(SIG_BLOCK, [CHLD], [], 8) = 0
wait4(-1, strace: Process 29657 attached
 <unfinished ...>
[pid 29657] rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
[pid 29657] rt_sigaction(SIGTSTP, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
[pid 29657] rt_sigaction(SIGTTIN, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
[pid 29657] rt_sigaction(SIGTTOU, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
[pid 29657] setpgid(29657, 29657)       = 0
[pid 29657] rt_sigprocmask(SIG_BLOCK, [CHLD TSTP TTIN TTOU], [], 8) = 0
[pid 29657] ioctl(255, TIOCSPGRP, [29657]) = 0
[pid 29657] rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
[pid 29657] close(4)                    = 0
[pid 29657] read(3, "", 1)              = 0
[pid 29657] close(3)                    = 0
[pid 29657] rt_sigaction(SIGHUP, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGILL, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGTRAP, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGABRT, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGFPE, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGBUS, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGSEGV, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGSYS, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGPIPE, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGALRM, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGXCPU, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGXFSZ, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGVTALRM, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGUSR1, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGUSR2, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, NULL, 8) = 0
[pid 29657] rt_sigaction(SIGINT, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
[pid 29657] rt_sigaction(SIGQUIT, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
[pid 29657] rt_sigaction(SIGTERM, {SIG_DFL, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f230, [], SA_RESTORER|SA_RESTART, 0x7fa5496b64b0}, 8) = 0
[pid 29657] rt_sigaction(SIGCHLD, {SIG_DFL, [], SA_RESTORER|SA_RESTART, 0x7fa5496b64b0}, {0x447a20, [], SA_RESTORER|SA_RESTART, 0x7fa5496b64b0}, 8) = 0
[pid 29657] execve("./a.out", ["./a.out"], [/* 35 vars */]) = 0
[pid 29657] brk(NULL)                   = 0xed2000
[pid 29657] access("/etc/ld.so.nohwcap", F_OK) = -1 ENOENT (No such file or directory)
[pid 29657] mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fe4c8b2f000
[pid 29657] access("/etc/ld.so.preload", R_OK) = -1 ENOENT (No such file or directory)
[pid 29657] open("/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
[pid 29657] fstat(3, {st_mode=S_IFREG|0644, st_size=100148, ...}) = 0
[pid 29657] mmap(NULL, 100148, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7fe4c8b16000
[pid 29657] close(3)                    = 0
[pid 29657] access("/etc/ld.so.nohwcap", F_OK) = -1 ENOENT (No such file or directory)
[pid 29657] open("/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
[pid 29657] read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0P\t\2\0\0\0\0\0"..., 832) = 832
[pid 29657] fstat(3, {st_mode=S_IFREG|0755, st_size=1864888, ...}) = 0
[pid 29657] mmap(NULL, 3967392, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fe4c8543000
[pid 29657] mprotect(0x7fe4c8702000, 2097152, PROT_NONE) = 0
[pid 29657] mmap(0x7fe4c8902000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1bf000) = 0x7fe4c8902000
[pid 29657] mmap(0x7fe4c8908000, 14752, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7fe4c8908000
[pid 29657] close(3)                    = 0
[pid 29657] mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fe4c8b15000
[pid 29657] mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fe4c8b14000
[pid 29657] mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fe4c8b13000
[pid 29657] arch_prctl(ARCH_SET_FS, 0x7fe4c8b14700) = 0
[pid 29657] mprotect(0x7fe4c8902000, 16384, PROT_READ) = 0
[pid 29657] mprotect(0x600000, 4096, PROT_READ) = 0
[pid 29657] mprotect(0x7fe4c8b31000, 4096, PROT_READ) = 0
[pid 29657] munmap(0x7fe4c8b16000, 100148) = 0
[pid 29657] getpid()                    = 29657
[pid 29657] fstat(1, {st_mode=S_IFCHR|0620, st_rdev=makedev(136, 9), ...}) = 0
[pid 29657] brk(NULL)                   = 0xed2000
[pid 29657] brk(0xef3000)               = 0xef3000
[pid 29657] write(1, "x29657", 6)       = 6
[pid 29657] exit_group(0)               = ?
[pid 29657] +++ exited with 0 +++
<... wait4 resumed> [{WIFEXITED(s) && WEXITSTATUS(s) == 0}], WSTOPPED|WCONTINUED, NULL) = 29657
rt_sigprocmask(SIG_BLOCK, [CHLD TSTP TTIN TTOU], [CHLD], 8) = 0
ioctl(255, TIOCSPGRP, [29571])          = 0
rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
ioctl(255, TCGETS, {B38400 opost isig icanon echo ...}) = 0
ioctl(255, TIOCGWINSZ, {ws_row=45, ws_col=181, ws_xpixel=0, ws_ypixel=0}) = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=29657, si_uid=1000, si_status=0, si_utime=0, si_stime=0} ---
wait4(-1, 0x7fff85e9a8d0, WNOHANG|WSTOPPED|WCONTINUED, NULL) = -1 ECHILD (No child processes)
rt_sigreturn({mask=[]})                 = 0
rt_sigaction(SIGINT, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigprocmask(SIG_BLOCK, [CHLD TSTP TTIN TTOU], [], 8) = 0
ioctl(255, TIOCSPGRP, [29571])          = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
rt_sigaction(SIGINT, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
ioctl(0, TIOCGWINSZ, {ws_row=45, ws_col=181, ws_xpixel=0, ws_ypixel=0}) = 0
ioctl(0, TIOCSWINSZ, {ws_row=45, ws_col=181, ws_xpixel=0, ws_ypixel=0}) = 0
ioctl(0, TCGETS, {B38400 opost isig icanon echo ...}) = 0
ioctl(0, TCGETS, {B38400 opost isig icanon echo ...}) = 0
ioctl(0, SNDCTL_TMR_STOP or TCSETSW, {B38400 opost isig -icanon -echo ...}) = 0
ioctl(0, TCGETS, {B38400 opost isig -icanon -echo ...}) = 0
rt_sigprocmask(SIG_BLOCK, [HUP INT QUIT ALRM TERM TSTP TTIN TTOU], [], 8) = 0
rt_sigaction(SIGINT, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f6e0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTERM, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f230, [], SA_RESTORER|SA_RESTART, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGHUP, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f960, [HUP INT ILL TRAP ABRT BUS FPE USR1 SEGV USR2 PIPE ALRM TERM XCPU XFSZ VTALRM SYS], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGQUIT, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGQUIT, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGALRM, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {0x45f960, [HUP INT ILL TRAP ABRT BUS FPE USR1 SEGV USR2 PIPE ALRM TERM XCPU XFSZ VTALRM SYS], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTSTP, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTSTP, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTTOU, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTTOU, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTTIN, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigaction(SIGTTIN, {SIG_IGN, [], SA_RESTORER, 0x7fa5496b64b0}, {0x4acdd0, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
rt_sigaction(SIGWINCH, {0x4ac3f0, [], SA_RESTORER|SA_RESTART, 0x7fa5496b64b0}, {0x45f220, [], SA_RESTORER, 0x7fa5496b64b0}, 8) = 0
write(2, "\33]0;jason@jason-VirtualBox: ~/te"..., 92) = 92
read(0, ^@"\0", 1)                        = 1
read(0,
```
