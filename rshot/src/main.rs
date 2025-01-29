mod slurp;
mod grim;

use std::env::args;
use std::collections::HashSet;
use grim::{screenshot, CaptureMode, SaveMode};

fn main() {
    let args: HashSet<String> = HashSet::from_iter(args().into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
    let savemode = if args.contains("--save-only") { SaveMode::SaveToFile } else if args.contains("--save") { SaveMode::CopyAndSave } else { SaveMode::Clipboard };
    let capturemode = if args.contains("region") { CaptureMode::Region } else if args.contains("colour") { CaptureMode::Pixel } else { CaptureMode::Everything };
    let _ = screenshot(savemode, capturemode);
}
