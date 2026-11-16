#[derive(Default, Debug, PartialEq)]
pub struct Options {
    pub snapshot: Option<String>,
    pub smoke_frames: Option<u32>,
    pub x11: bool,
    pub help: bool,
}

impl Options {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut args = args.into_iter().peekable();
        let mut options = Self::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => options.help = true,
                "--x11" => {
                    if !cfg!(target_os = "linux") {
                        return Err("--x11 is Linux-only".into());
                    }
                    options.x11 = true;
                }
                "--snapshot" => {
                    if options.snapshot.is_some() {
                        return Err("--snapshot supplied twice".into());
                    }
                    let path = args
                        .next()
                        .filter(|p| !p.starts_with('-'))
                        .ok_or("--snapshot requires an output .ppm path")?;
                    options.snapshot = Some(path);
                }
                "--smoke" => {
                    if options.smoke_frames.is_some() {
                        return Err("--smoke supplied twice".into());
                    }
                    let count = if args.peek().is_some_and(|value| !value.starts_with('-')) {
                        args.next()
                            .unwrap()
                            .parse::<u32>()
                            .map_err(|_| "--smoke requires a positive integer frame count")?
                    } else {
                        120
                    };
                    if count == 0 {
                        return Err("--smoke requires a positive frame count".into());
                    }
                    options.smoke_frames = Some(count);
                }
                _ => return Err(format!("unknown argument: {arg}. Use --help for usage.")),
            }
        }
        if options.snapshot.is_some() && options.smoke_frames.is_some() {
            return Err("--snapshot and --smoke are separate checks; run them separately".into());
        }
        Ok(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(args: &[&str]) -> Result<Options, String> {
        Options::parse(args.iter().map(|s| s.to_string()))
    }
    #[test]
    fn accepts_smoke_defaults_and_explicit_count() {
        assert_eq!(parse(&["--smoke"]).unwrap().smoke_frames, Some(120));
        assert_eq!(parse(&["--smoke", "30"]).unwrap().smoke_frames, Some(30));
    }
    #[test]
    fn invalid_options_do_not_silently_start_a_window() {
        for args in [
            vec!["--smoke", "0"],
            vec!["--smoke", "abc"],
            vec!["--smoke", "-1"],
            vec!["--snapshot"],
            vec!["--snapshot", "--smoke"],
            vec!["--typo"],
            vec!["--smoke", "10", "--snapshot", "test.ppm"],
            vec!["--smoke", "--smoke"],
        ] {
            assert!(parse(&args).is_err(), "accepted {args:?}");
        }
    }
}
