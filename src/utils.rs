#[allow(unused_macros)]
macro_rules! log {
    ($e:expr) => {{
        if cfg!(debug_assertions) {
            if let Ok(mut file) = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("phetch.log")
        {
            file.write($e.as_ref());
            file.write(b"\n");
        }
    }
    }};
    ($e:expr, $($y:expr),*) => {
        if cfg!(debug_assertions) {
            log!(format!($e, $($y),*));
        }
    };
}

macro_rules! error {
    ($e:expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $e)
    };
    ($e:expr, $($y:expr),*) => {
        error!(format!($e, $($y),*));
    };
}
