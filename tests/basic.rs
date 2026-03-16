#[cfg(unix)]
#[test]
fn test_fork_detection() {
    let mut guard = forkguard::new();
    assert!(!guard.detected_fork(), "initial check should be false");

    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // Child process: detect fork and exit with 0 on success.
        //
        // We use exit code to communicate the result back to the parent process, because standard
        // assertions in a forked child may not be correctly reported by the test runner.
        if !guard.detected_fork() {
            std::process::exit(1); // first call should detect a call
        }
        if guard.detected_fork() {
            std::process::exit(2); // second call should not detect a call
        }
        std::process::exit(0);
    } else if pid > 0 {
        // Parent process
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0) };
        assert!(libc::WIFEXITED(status), "child should exit normally");
        assert_eq!(
            libc::WEXITSTATUS(status),
            0,
            "child should have detected the fork"
        );

        assert!(!guard.detected_fork(), "parent should not detect a fork");
    } else {
        panic!("fork failed");
    }
}
