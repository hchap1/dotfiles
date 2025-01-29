use std::process::Command;
use crate::slurp::{Region, SlurpMode};
use std::fs::read_dir;
use home::home_dir;

pub enum SaveMode {
    Clipboard,
    SaveToFile,
    CopyAndSave
}

#[derive(PartialEq, Eq)]
pub enum CaptureMode {
    Everything,
    Region,
    Pixel
}

fn largest_screenshot_number(directory: &str) -> usize {
    let dir = match home_dir() {
        Some(path) => format!("{}/{directory}", path.display()),
        None => return 0
    };

    println!("Using dir: {dir}");

    let mut number: usize = 0;
    let entries = match read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("ERR: {e:?}");
            return 0;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return 0
        };

        if entry.path().is_file() {
            println!("{}", entry.file_name().to_string_lossy().to_string());
            if entry.file_name().to_string_lossy().to_string().starts_with("rshot_") {
                let file_number = match entry.file_name().to_string_lossy().to_string().strip_suffix(".png").unwrap().split("_").nth(1) {
                    Some(slice) => match slice.parse::<usize>() {
                        Ok(number) => number,
                        Err(_) => return 0
                    }
                    None => return 0
                };

                if file_number > number { number = file_number; }
            }
        }
    }

    number + 1
}

pub fn screenshot(savemode: SaveMode, capturemode: CaptureMode) -> Result<(), ()> {
    let region: String = format!("\"{}\"", match capturemode {
        CaptureMode::Pixel => Region::slurp(SlurpMode::Point).unwrap().fmt(),
        CaptureMode::Region => match Region::slurp(SlurpMode::Region) {
            Ok(region) => region.fmt(),
            Err(_) => return Err(())
        }
        CaptureMode::Everything => String::new()
    });

    let start_flag = match capturemode {
        CaptureMode::Pixel => vec!["-g".to_string(), format!("{region}", ),
            "-t".to_string(), "ppm".to_string(), "-".to_string(), "|".to_string(), "magick".to_string(), "-".to_string(),
            "-format".to_string(), "'%[pixel:p{0,0}]'".to_string(), "txt:-".to_string()],
        CaptureMode::Region => vec!["-g".to_string(), format!("{}", region)],
        CaptureMode::Everything => vec![]
    };

    let end_flag = if capturemode == CaptureMode::Pixel { vec![] } else { match savemode {
        SaveMode::Clipboard => String::from("- | wl-copy"),
        SaveMode::SaveToFile => format!("- | tee ~/Pictures/screenshots/rshot_{}.png >> /dev/null", largest_screenshot_number("Pictures/screenshots")),
        SaveMode::CopyAndSave => format!("- | tee ~/Pictures/screenshots/rshot_{}.png >> /dev/null | wl-copy", largest_screenshot_number("Pictures/screenshots"))
    }.split(" ").map(|x| x.to_string()).collect::<Vec<String>>()};

    if capturemode == CaptureMode::Pixel {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("grim {} {}", start_flag.join(" "), end_flag.join(" ")))
            .output().expect("Grim is not installed.");
        if !output.status.success() {
            eprintln!("Failed to run command.");
        }
        let lines = output.stdout.iter().map(|x| *x as char).collect::<String>().split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
        if lines.len() > 1 {
            let value = lines[1].split(" ").map(|x| x.to_string()).nth(1).unwrap();
            let _ = Command::new("wl-copy").arg(value).spawn().expect("Could not copy RGB value.");
        }
    } else {
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!("grim {} {}", start_flag.join(" "), end_flag.join(" ")))
            .spawn().expect("Grim is not installed.");
    }

    Ok(())
}
