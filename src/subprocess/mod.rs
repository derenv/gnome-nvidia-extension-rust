// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing subprocess module
 *
 * Made:
 * 15/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * <https://blog.logrocket.com/a-practical-guide-to-async-in-rust/>
 */
pub mod subprocess {
    // Imports
    use glib::Bytes;
    use gtk::{gio, glib, prelude::*};
    use std::ffi::OsStr;

    /**
     * Name:
     * exec_check_async
     *
     * Description:
     * Execute a command asynchronously and check the exit status
     *
     * If given, @cancellable can be used to stop the process before it finishes.
     *
     * <https://gtk-rs.org/gtk-rs-core/stable/0.14/docs/src/gio/auto/subprocess.rs.html>
     *
     * Made:
     * 15/09/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn exec_check_async(
        argv: &[&OsStr],
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        // Create subprocess
        match gio::Subprocess::newv(argv, gio::SubprocessFlags::NONE) {
            Err(err) => Err(err),
            // Run subprocess
            Ok(proc) => match proc.wait_async(cancellable, |_| ()) {
                _ => Ok(()),
            },
        }
    }

    /**
     * Name:
     * exec_communicate_sync
     *
     * Description:
     * Execute a command and return any output
     *
     * If given, @cancellable can be used to stop the process before it finishes.
     *
     * <https://gtk-rs.org/gtk-rs-core/stable/0.14/docs/src/gio/auto/subprocess.rs.html>
     *
     * Made:
     * 15/09/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn exec_communicate_sync(
        argv: &[&OsStr],
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(Option<Bytes>, Option<Bytes>), glib::Error> {
        // Create subprocess
        match gio::Subprocess::newv(argv, gio::SubprocessFlags::STDOUT_PIPE) {
            Err(err) => Err(err),
            // Run subprocess
            Ok(proc) => match proc.communicate(None, cancellable) {
                Err(err) => Err(err),
                Ok(buffers) => match buffers {
                    (None, None) => Ok((None, None)),
                    (None, Some(stderr_buffer)) => Ok((None, Some(stderr_buffer))),
                    (Some(stdout_buffer), None) => Ok((Some(stdout_buffer), None)),
                    (Some(stdout_buffer), Some(stderr_buffer)) => {
                        Ok((Some(stdout_buffer), Some(stderr_buffer)))
                    }
                },
            },
        }
    }

    /**
     * Name:
     * exec_communicate
     *
     * Description:
     * Execute a command asynchronously and check the exit status
     *
     * If given, @cancellable can be used to stop the process before it finishes.
     *
     * <https://gtk-rs.org/gtk-rs-core/stable/0.14/docs/src/gio/auto/subprocess.rs.html>
     *
     * Made:
     * 27/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn exec_communicate_async<
        Q: FnOnce(Result<(Option<glib::Bytes>, Option<glib::Bytes>), glib::Error>) + Send + 'static,
    >(
        argv: &[&OsStr],
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        // Callback? that way could modify settings....
        callback: Q, //dyn FnOnce(Result<(Option<glib::Bytes>, Option<glib::Bytes>), glib::Error>) + 'static
    ) -> Result<(), glib::Error> {
        // Create subprocess
        match gio::Subprocess::newv(argv, gio::SubprocessFlags::STDOUT_PIPE) {
            Err(err) => Err(err),
            // Run subprocess
            Ok(proc) => {
                proc.communicate_async(None, cancellable, callback);

                Ok(())
            }
        }
    }
}
