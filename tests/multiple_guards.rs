#[cfg(unix)]
#[test]
fn test_multiple_guards() {
    let mut guard1 = forkguard::new();
    let mut guard2 = forkguard::new();

    assert!(!guard1.detected_fork());
    assert!(!guard2.detected_fork());

    let pid = unsafe { libc::fork() };
    if pid == 0 {
        if !guard1.detected_fork() {
            std::process::exit(1);
        }
        if !guard2.detected_fork() {
            std::process::exit(2);
        }

        if guard1.detected_fork() {
            std::process::exit(3);
        }
        if guard2.detected_fork() {
            std::process::exit(4);
        }

        std::process::exit(0);
    } else if pid > 0 {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0) };
        assert!(libc::WIFEXITED(status));
        assert_eq!(
            libc::WEXITSTATUS(status),
            0,
            "one of the guards failed to detect fork in child"
        );

        assert!(!guard1.detected_fork());
        assert!(!guard2.detected_fork());
    }
}
