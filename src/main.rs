use std::{fs, os::fd::AsRawFd};

use nix::{
    libc::AT_FDCWD,
    sys::fanotify::{
        EventFFlags, Fanotify, FanotifyResponse, InitFlags, MarkFlags, MaskFlags, Response,
    },
};

fn main() {
    let notify = Fanotify::init(
        InitFlags::FAN_CLOEXEC | InitFlags::FAN_CLASS_CONTENT, // | InitFlags::FAN_NONBLOCK,
        EventFFlags::O_RDONLY | EventFFlags::O_LARGEFILE,
    )
    .unwrap();

    notify
        .mark(
            MarkFlags::FAN_MARK_ADD | MarkFlags::FAN_MARK_MOUNT,
            MaskFlags::FAN_OPEN_PERM | MaskFlags::FAN_CLOSE_WRITE,
            Some(AT_FDCWD),
            Some("."),
        )
        .unwrap();

    loop {
        for event in notify.read_events().unwrap().iter() {
            print!("Event: ");
            if event.mask().contains(MaskFlags::FAN_OPEN_PERM) {
                notify
                    .write_response(FanotifyResponse::new(
                        event.fd().unwrap(),
                        Response::FAN_ALLOW,
                    ))
                    .unwrap();

                print!("OPEN ");
            } else if event.mask().contains(MaskFlags::FAN_CLOSE_WRITE) {
                print!("CLOSE_WRITE ");
            }

            let fd_path = format!("/proc/self/fd/{}", event.fd().unwrap().as_raw_fd());
            let file_name = fs::read_link(fd_path).unwrap();

            print!("file '{}' ", file_name.to_str().unwrap());

            let exe_path = format!("/proc/{}/exe", event.pid());
            let exe_name = fs::read_link(exe_path).unwrap();

            println!("from '{}'", exe_name.to_str().unwrap());
        }
    }
}
