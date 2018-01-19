//! QEMU Display settings.

/// VNC Display settings.
pub struct Vnc {
    host: String,
    display: u16,
    ws_port: Option<u16>,
}

impl Vnc {
    /// Create a new VNC display configuration.
    pub fn new<S: Into<String>>(host: S, display: u16) -> Vnc {
        Vnc {
            host: host.into(),
            display: display,
            ws_port: None,
        }
    }

    /// Create a new VNC display configuration with an additional websocket port.
    pub fn with_websocket<S: Into<String>>(host: S, display: u16, ws_port: u16) -> Vnc {
        Vnc {
            host: host.into(),
            display: display,
            ws_port: Some(ws_port),
        }
    }
}

/// Represnts the settings of a display used with a machine.
pub enum Display {
    None,
    Sdl,
    Vnc(Vnc),
}

impl super::IntoArguments for Display {
    fn into_arguments(self) -> Vec<String> {
        let mut args = vec![String::from("-display")];

        let param = match self {
            Display::None => String::from("none"),
            Display::Sdl => String::from("sdl"),
            Display::Vnc(vnc) => {
                let mut param = format!("vnc={}:{}", vnc.host, vnc.display);

                if let Some(ws_port) = vnc.ws_port {
                    param.push_str(format!(",websocket={}", ws_port).as_str());
                }

                param
            },
        };

        args.push(param);
        args
    }
}
