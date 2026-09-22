//! Makes sure that only a single instance of the app is running.
//!
//! Without this every launch would add another tray icon and another set of
//! monitor threads fighting over the brightness.

use windows::core::{w, HRESULT, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE};
use windows::Win32::System::Threading::{CreateMutexW, OpenMutexW, MUTEX_ALL_ACCESS};

use crate::Result;

/// The name of the mutex that marks a running instance. It is session local,
/// so one instance per logged in user is allowed.
const MUTEX_NAME: PCWSTR = w!("rusty-twinkle-tray-single-instance");

/// Holds the single instance lock. Dropping it releases the lock.
pub struct SingleInstance(HANDLE);

/// Tries to become the one and only instance.
///
/// Returns `Ok(None)` if another instance is already running.
pub fn acquire() -> Result<Option<SingleInstance>> {
    // OpenMutexW is the reliable way to detect an existing instance: it fails
    // with ERROR_FILE_NOT_FOUND when nobody holds the mutex.
    if let Ok(existing) = unsafe { OpenMutexW(MUTEX_ALL_ACCESS, false, MUTEX_NAME) } {
        unsafe {
            let _ = CloseHandle(existing);
        }
        return Ok(None);
    }

    let handle = unsafe { CreateMutexW(None, false, MUTEX_NAME) }?;
    // GetLastError returns Err(...) for anything but ERROR_SUCCESS.
    if let Err(err) = unsafe { GetLastError() } {
        if err.code() == HRESULT::from_win32(ERROR_ALREADY_EXISTS.0) {
            // Lost the race against another instance that created the mutex first.
            unsafe {
                let _ = CloseHandle(handle);
            }
            return Ok(None);
        }
    }

    log::debug!("Acquired the single instance lock");
    Ok(Some(SingleInstance(handle)))
}

impl Drop for SingleInstance {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}
