use std::io::Error;
use std::process::Command;

struct CinnamonSettings {}
impl CinnamonSettings {
    /// Retrieves a cinnamon configuration via dconf.
    ///
    /// # Example
    /// ```
    /// let config = CinnamonSettings.get("number-workspaces").unwrap();
    ///
    /// assert_eq!(config, String::from("0"));
    /// ```
    fn get(setting: &str) -> Result<String, Error> {
        Command::new("dconf")
            .arg("read")
            .arg(format!("/org/cinnamon/"))
            .output()
            .map(
                |o| String::from_utf8(o.stdout).unwrap().trim().to_string(), // WARN: prayge
            )
    }

    /// Retrieves the dconf configuration `/org/cinnamon/panels-enabled`.
    ///
    /// Returns a tuple vector which contain the following information about each panel respectively:
    ///  - the panel id
    ///  - the monitor number (0 for primary) the panel is on
    ///  - position on the monitor the panel resides on (top, bottom, left, right)
    // NOTE: my ocd tells me to seperate these
    pub fn get_enabled_panels() -> Result<Vec<(u8, u8, String)>, Error> {
        let conf = Self::get("panels-enabled");
        if conf.is_err() {
            return Err(conf.unwrap_err());
        }
        let mut conf = conf.unwrap();
        conf.pop(); // '['
        conf.remove(0); // ']'
        Ok(conf
            .split(',')
            .map(|s| {
                let mut s = s.split(':');
                // thats a lotta unwraps
                (
                    s.next().unwrap().parse().unwrap(),
                    s.next().unwrap().parse().unwrap(),
                    s.next().unwrap().to_string(),
                )
            })
            .collect())
    }
}
