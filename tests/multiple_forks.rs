#[cfg(unix)]
#[test]
fn test_multiple_forks() {
    let mut guard = forkguard::new();
    assert!(!guard.detected_fork());

    // First fork
    let pid1 = unsafe { libc::fork() };
    if pid1 == 0 {
        if !guard.detected_fork() {
            std::process::exit(1);
        }
        if guard.detected_fork() {
            std::process::exit(2);
        }
        std::process::exit(0);
    }

    let mut status1 = 0;
    unsafe { libc::waitpid(pid1, &mut status1, 0) };
    assert!(libc::WIFEXITED(status1));
    assert_eq!(libc::WEXITSTATUS(status1), 0);
    assert!(!guard.detected_fork());

    // Second fork
    let pid2 = unsafe { libc::fork() };
    if pid2 == 0 {
        if !guard.detected_fork() {
            std::process::exit(1);
        }
        if guard.detected_fork() {
            std::process::exit(2);
        }
        std::process::exit(0);
    }

    let mut status2 = 0;
    unsafe { libc::waitpid(pid2, &mut status2, 0) };
    assert!(libc::WIFEXITED(status2));
    assert_eq!(libc::WEXITSTATUS(status2), 0);
    assert!(!guard.detected_fork());
}
