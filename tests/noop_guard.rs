#[test]
fn test_noop_guard() {
    let mut guard = forkguard::noop::Guard::default();
    assert!(!guard.detected_fork());

    #[cfg(unix)]
    {
        let pid = unsafe { libc::fork() };
        if pid == 0 {
            if guard.detected_fork() {
                std::process::exit(1);
            }
            std::process::exit(0);
        } else if pid > 0 {
            let mut status = 0;
            unsafe { libc::waitpid(pid, &mut status, 0) };
            assert!(libc::WIFEXITED(status));
            assert_eq!(libc::WEXITSTATUS(status), 0);
            assert!(!guard.detected_fork());
        }
    }
}
