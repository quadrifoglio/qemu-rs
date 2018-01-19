//! QEMU Display settings.

/// VNC Display settings.
pub struct Vnc {
    host: String,
    display: u16,
    ws_port: Option<u16>,
    password: bool,
}

impl Vnc {
    /// Create a new VNC display configuration.
    pub fn new<S: Into<String>>(host: S, display: u16) -> Vnc {
        Vnc {
            host: host.into(),
            display: display,
            ws_port: None,
            password: false,
        }
    }

    /// Create a new VNC display configuration with an additional websocket port.
    pub fn with_websocket<S: Into<String>>(host: S, display: u16, ws_port: u16) -> Vnc {
        Vnc {
            host: host.into(),
            display: display,
            ws_port: Some(ws_port),
            password: false,
        }
    }

    /// Specify wether the VNC server should require a password. The password must be set separetly
    /// using the QEMU Monitor.
    pub fn use_password(&mut self, passwd: bool) {
        self.password = passwd;
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
                    param.push_str(&format!(",websocket={}", ws_port));
                }

                if vnc.password == true {
                    param.push_str(&String::from(",password"));
                }

                param
            },
        };

        args.push(param);
        args
    }
}

/// Type of VGA card to emulate.
pub enum Vga {
    None,
    Cirrus,
    Std,
    VMWare,
    Qxl,
    Tcx,
    Cg3,
    Virtio,
}

impl super::IntoArguments for Vga {
    fn into_arguments(self) -> Vec<String> {
        let mut args = vec![String::from("-vga")];

        args.push(match self {
            Vga::None => String::from("none"),
            Vga::Cirrus => String::from("cirrus"),
            Vga::Std => String::from("std"),
            Vga::VMWare => String::from("vmware"),
            Vga::Qxl => String::from("qxl"),
            Vga::Tcx => String::from("tcx"),
            Vga::Cg3 => String::from("cg3"),
            Vga::Virtio => String::from("virtio"),
        });

        args
    }
}
