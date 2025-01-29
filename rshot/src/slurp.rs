use std::process::{Command, Stdio};

pub struct Region {
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

pub enum SlurpMode {
    Point,
    Region
}

impl Region {
    pub fn slurp(mode: SlurpMode) -> Result<Self, ()> {
        let output = match mode {
            SlurpMode::Region => Command::new("slurp").stdout(Stdio::piped()).output().expect("Slurp is not installed."),
            SlurpMode::Point => Command::new("slurp").arg("-p").stdout(Stdio::piped()).output().expect("Slurp is not installed.")
        };

        match output.status.success() {
            true => Ok(Self::from(String::from_utf8_lossy(&output.stdout).to_string())),
            false => Err(())
        }
    }

    pub fn fmt(&self) -> String {
        format!("{},{} {}x{}", self.x + 2, self.y + 2, self.w - 4, self.h - 4)
    }
}

impl From<String> for Region {
    fn from(other: String) -> Self {
        let data = other.strip_suffix('\n').unwrap().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        assert!(data.len() <= 3, "Improperly formatted geometry: Expected 'X,Y WxH'");
        let top_left = data[0].split(',').map(|x| match x.parse::<usize>() {
            Ok(ordinate) => ordinate,
            Err(_) => 0
        }).collect::<Vec<usize>>();

        let size = data[1].split('x').map(|x| match x.parse::<usize>() {
            Ok(ordinate) => ordinate,
            Err(_) => 0
        }).collect::<Vec<usize>>();

        Self { x: top_left[0], y: top_left[1], w: size[0], h: size[1] }
    }
}
