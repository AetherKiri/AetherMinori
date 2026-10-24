//! Headless acceptance run.
//!
//! Opens a real Minori installation, ticks it through the provider ABI and
//! writes a PNG per sampled frame, so a human or a harness can look at what the
//! runtime actually drew. This is the lightweight backend the ef integration is
//! verified with; it does not replace the Aether host, it drives the same
//! provider the host drives.
//!
//! Usage:
//!   minori-headless GAME_ROOT OUTPUT_DIR [--frames N] [--delta MS] [--clicks N]
//!                   [--language e|j] [--start SCRIPT] [--every N] [--choose 0,1,...]

use std::env;
use std::io::Write;
use std::path::PathBuf;

use aether_minori_runtime::headless::Headless;
use aether_minori_runtime::text::TextRenderer;

fn main() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let root = PathBuf::from(args.next().ok_or(
        "usage: minori-headless GAME_ROOT OUTPUT_DIR [--frames N] [--delta MS] [--clicks N] \
         [--language e|j] [--start SCRIPT] [--every N] [--choose 0,1,...]",
    )?);
    let output = PathBuf::from(args.next().ok_or("missing OUTPUT_DIR")?);
    let mut frames = 120usize;
    let mut every = 20usize;
    let mut delta = 16u32;
    let mut clicks = 0usize;
    let mut language = "j".to_string();
    let mut start: Option<String> = None;
    // Zero-based answers by branch ordinal; unspecified branches use option zero.
    let mut choices: Vec<usize> = Vec::new();
    while let Some(argument) = args.next() {
        let text = argument.to_string_lossy().into_owned();
        let mut value = || -> Result<String, String> {
            args.next()
                .map(|v| v.to_string_lossy().into_owned())
                .ok_or_else(|| format!("{text} requires a value"))
        };
        match text.as_str() {
            "--frames" => frames = value()?.parse().map_err(|e| format!("{e}"))?,
            "--every" => every = value()?.parse::<usize>().map_err(|e| format!("{e}"))?.max(1),
            "--choose" => {
                choices = value()?
                    .split(',')
                    .map(|part| part.trim().parse::<usize>().map_err(|e| format!("{e}")))
                    .collect::<Result<Vec<_>, _>>()?
            }
            "--delta" => delta = value()?.parse().map_err(|e| format!("{e}"))?,
            "--clicks" => clicks = value()?.parse().map_err(|e| format!("{e}"))?,
            "--language" => language = value()?,
            "--start" => start = Some(value()?),
            other => return Err(format!("unknown argument {other}")),
        }
    }
    std::fs::create_dir_all(&output).map_err(|e| format!("create {}: {e}", output.display()))?;
    let writable = output.join("saves");
    std::fs::create_dir_all(&writable).map_err(|e| format!("create {}: {e}", writable.display()))?;

    let mut runtime = Headless::open(&root, &writable)?;
    runtime.set_option("minori.language", &language)?;
    if let Some(script) = &start {
        runtime.set_option("minori.start", script)?;
        runtime.restart()?;
    }

    // The runtime emits minori_message platform requests; the host turns them
    // into labels. The headless backend has no host, so it rasterises the same
    // strings into the frame -- that is what makes the dialogue verifiable.
    let text = TextRenderer::system().ok();
    println!("text_renderer={}", text.is_some());
    let mut message: Option<(String, String)> = None;
    let mut requests: Vec<(String, String)> = Vec::new();
    // Persist each observation immediately, including when a later tick fails.
    let mut trace = std::fs::File::create(output.join("story-trace.txt"))
        .map_err(|e| format!("create trace: {e}"))?;
    let mut branches = 0usize;

    let mut saved = 0usize;
    let mut last_serial = u64::MAX;
    for index in 0..frames {
        if let Err(error) = runtime.tick(delta) {
            writeln!(trace, "error\t{index}\t{error}\t{}", runtime.debug_info())
                .map_err(|e| format!("write trace: {e}"))?;
            return Err(error);
        }
        if clicks > 0 && index % (frames / clicks.max(1)).max(1) == 0 {
            runtime.press()?;
        }
        for (name, value) in aether_minori_runtime::headless::take_requests() {
            requests.push((name.clone(), value.clone()));
            match name.as_str() {
                "minori_message" => {
                    if value.is_empty() {
                        message = None;
                    } else {
                        let fields: Vec<&str> = value.split('\t').collect();
                        if fields.len() >= 2 {
                            message = Some((fields[0].to_string(), fields[1].to_string()));
                        }
                    }
                }
                "minori_select" => {
                    // The engine sits in WaitingSelect until the host posts a
                    // choice back; the retail host shows a menu and returns the
                    // chosen index through submit_platform_response.
                    let options: Vec<&str> = value
                        .lines()
                        .filter(|line| !line.is_empty())
                        .collect();
                    // --choose indexes by branch ordinal, so a route can name
                    // exactly the decision at each fork. Branches past the list
                    // take the first option.
                    let wanted = choices.get(branches).copied().unwrap_or(0);
                    branches += 1;
                    if wanted >= options.len() {
                        return Err(format!("branch {branches}: option {wanted} outside {} choices", options.len()));
                    }
                    let index = wanted;
                    // Capture the actual waiting state before answering. This is a
                    // backend diagnostic menu, not a claim about the retail UI.
                    if let (Some(renderer), Some((width, height, mut rgba))) = (text.as_ref(), runtime.frame()) {
                        draw_choices(renderer, &mut rgba, width, height, &options, index);
                        write_png(&output.join(format!("branch-{branches:02}.png")), width, height, &rgba)?;
                    }
                    writeln!(trace, "branch\t{}\t{}", index,
                        options.get(index).copied().unwrap_or(""))
                        .map_err(|e| format!("write trace: {e}"))?;
                    runtime.submit_response("minori_select", &format!("index={index}"))?;
                }
                "minori_movie" => {
                    // Offline movie backend: decode every video/audio packet to a
                    // null sink, and preserve a real decoded first-frame screenshot.
                    // This consumes the clip without wall-clock playback or sound.
                    let path = value.split('\t').next().ok_or("movie request lacks a path")?;
                    let snapshot = output.join(format!("movie-{index:06}.png"));
                    let first = std::process::Command::new("ffmpeg")
                        .args(["-nostdin", "-v", "error", "-y", "-i", path, "-vf", "thumbnail=48", "-frames:v", "1"])
                        .arg(&snapshot).output().map_err(|e| format!("start movie decoder: {e}"))?;
                    if !first.status.success() {
                        return Err(format!("movie frame decode failed: {}", String::from_utf8_lossy(&first.stderr)));
                    }
                    let decoded = std::process::Command::new("ffmpeg")
                        .args(["-nostdin", "-v", "error", "-i", path, "-f", "null", "-"])
                        .output().map_err(|e| format!("start movie decoder: {e}"))?;
                    if !decoded.status.success() {
                        return Err(format!("movie stream decode failed: {}", String::from_utf8_lossy(&decoded.stderr)));
                    }
                    if !decoded.stderr.is_empty() {
                        // Preserve decoder-reported damaged packets; a successful
                        // process exit is not a claim of lossless media integrity.
                        writeln!(trace, "movie-warning\t{index}\t{}",
                            String::from_utf8_lossy(&decoded.stderr).replace('\n', " | "))
                            .map_err(|e| format!("write movie warning: {e}"))?;
                    }
                    writeln!(trace, "movie\t{index}\tdecoded-offline\t{path}")
                        .map_err(|e| format!("write movie trace: {e}"))?;
                    runtime.submit_response("minori_movie", "result=complete")?;
                }
                "minori_message_reveal" => {}
                _ => {}
            }
        }
        let debug = runtime.debug_info();
        let script = field(&debug, "script=");
        let state = field(&debug, "state=");
        let message_id = field(&debug, "message=");
        let panel_rect = panel_geometry(&debug);
        let line = format!("{index:04}	{script}	{message_id}	{state}");
        writeln!(trace, "{line}").map_err(|e| format!("write trace: {e}"))?;
        let finished = state == "Finished";
        // Only sample frames: a 3,000-tick run would otherwise write several
        // gigabytes of PNG and has no extra value over the trace.
        if (index % every == 0 || finished)
            && let Some((width, height, mut rgba)) = runtime.frame()
        {
            if let (Some(renderer), Some((speaker, body))) = (text.as_ref(), message.as_ref()) {
                draw_message(renderer, &mut rgba, width, height, speaker, body, panel_rect);
            }
            let path = output.join(format!("frame-{index:04}.png"));
            write_png(&path, width, height, &rgba)?;
            saved += 1;
            last_serial = index as u64;
        }
        if finished {
            println!("finished_at={index}");
            break;
        }
    }
    println!("frames_written={saved} last_index={last_serial}");
    trace.flush().map_err(|e| format!("flush trace: {e}"))?;
    println!("debug={}", runtime.debug_info());
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for (name, _) in &requests {
        *counts.entry(name.as_str()).or_default() += 1;
    }
    println!("requests={:?}", counts);
    for (_name, value) in requests.iter().filter(|(n, _)| n == "presentation_sprite").take(4) {
        println!("sprite -> {value}");
    }
    for (name, value) in requests.iter().filter(|(n, _)| n == "minori_message").take(3) {
        println!("{name} -> {value}");
    }
    Ok(())
}

fn field(debug: &str, prefix: &str) -> String {
    debug
        .split(' ')
        .find_map(|part| part.strip_prefix(prefix))
        .unwrap_or("-")
        .to_string()
}

/// Parse the panel=WxH+X+Y field out of the debug line.
fn panel_geometry(debug: &str) -> Option<(u32, u32, i32)> {
    let value = field(debug, "panel=");
    if value == "none" {
        return None;
    }
    let (size, rest) = value.split_once('+')?;
    let (_x, top) = rest.split_once('+')?;
    let (w, h) = size.split_once('x')?;
    Some((w.parse().ok()?, h.parse().ok()?, top.parse().ok()?))
}

#[allow(clippy::too_many_arguments)]
fn draw_message(
    renderer: &TextRenderer,
    rgba: &mut [u8],
    width: u32,
    height: u32,
    speaker: &str,
    body: &str,
    panel: Option<(u32, u32, i32)>,
) {
    // Place the text inside the panel the runtime actually drew. Without a
    // panel (the narration before the first .panel command) fall back to the
    // lower third of the surface.
    const LEFT: f32 = 64.0;
    const MARGIN: f32 = 72.0;
    let max_width = width as f32 - LEFT - MARGIN;
    let body_size = 24.0;
    let speaker_size = 22.0;
    let body_top = match panel {
        Some((_w, _h, top)) => top as f32 + 74.0,
        None => height as f32 - 158.0,
    };
    if !speaker.is_empty() {
        renderer.draw(
            rgba,
            width,
            height,
            LEFT,
            body_top - 30.0,
            max_width,
            speaker_size,
            [255, 236, 196, 255],
            speaker,
        );
    }
    renderer.draw(
        rgba,
        width,
        height,
        LEFT,
        body_top,
        max_width,
        body_size,
        [255, 255, 255, 255],
        body,
    );
}

fn draw_choices(renderer: &TextRenderer, rgba: &mut [u8], width: u32, height: u32, options: &[&str], selected: usize) {
    let top = height.saturating_sub(options.len() as u32 * 56 + 40) / 2;
    let left = width / 8;
    let right = width - left;
    for (index, option) in options.iter().enumerate() {
        let y = top + index as u32 * 56;
        for row in y..(y + 48).min(height) {
            for col in left..right {
                let offset = ((row * width + col) * 4) as usize;
                let color = if index == selected { [40, 70, 110, 255] } else { [20, 20, 35, 255] };
                rgba[offset..offset + 4].copy_from_slice(&color);
            }
        }
        renderer.draw(rgba, width, height, left as f32 + 20.0, y as f32 + 32.0,
            (right - left - 40) as f32, 26.0, [255,255,255,255], &format!("{}  {option}", index + 1));
    }
}

fn write_png(path: &std::path::Path, width: u32, height: u32, rgba: &[u8]) -> Result<(), String> {
    let file = std::fs::File::create(path).map_err(|e| format!("create {}: {e}", path.display()))?;
    let writer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(|e| format!("png header: {e}"))?;
    writer
        .write_image_data(rgba)
        .map_err(|e| format!("png data: {e}"))?;
    Ok(())
}
