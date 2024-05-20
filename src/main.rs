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
            if event.mask().contains(MaskFlags::FAN_OPEN_PERM) {
                notify
                    .write_response(FanotifyResponse::new(
                        event.fd().unwrap(),
                        Response::FAN_ALLOW,
                    ))
                    .unwrap();
            }
            println!("{event:#?}");
        }
    }
}
