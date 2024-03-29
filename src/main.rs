use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
        let mut config = tun::Configuration::default();
        config.address((192, 168, 79, 10))
               .netmask((255, 255, 255, 0))
               .up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
                config.packet_information(true);
        });

        let mut dev = tun::create(&config).unwrap();
        let mut buf = [0; 4096];

        loop {
                let amount = dev.read(&mut buf).unwrap();
                println!("{:?}", &buf[0 .. amount]);
        }
}
