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
//!                   [--language e|j] [--start SCRIPT]

use std::env;
use std::path::PathBuf;

use aether_minori_runtime::headless::Headless;
use aether_minori_runtime::text::TextRenderer;

fn main() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let root = PathBuf::from(args.next().ok_or(
        "usage: minori-headless GAME_ROOT OUTPUT_DIR [--frames N] [--delta MS] [--clicks N] \
         [--language e|j] [--start SCRIPT]",
    )?);
    let output = PathBuf::from(args.next().ok_or("missing OUTPUT_DIR")?);
    let mut frames = 120usize;
    let mut every = 20usize;
    let mut delta = 16u32;
    let mut clicks = 0usize;
    let mut language = "j".to_string();
    let mut start: Option<String> = None;
    // Which option answers each branch, cycled. Empty means always the first.
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
    let mut trace: Vec<String> = Vec::new();
    let mut branches = 0usize;

    let mut saved = 0usize;
    let mut last_serial = u64::MAX;
    for index in 0..frames {
        runtime.tick(delta)?;
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
                    let index = wanted.min(options.len().saturating_sub(1));
                    trace.push(format!(
                        "branch\t{}\t{}",
                        index,
                        options.get(index).copied().unwrap_or("")
                    ));
                    runtime.submit_response("minori_select", &format!("index={index}"))?;
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
        if trace.last() != Some(&line) {
            trace.push(line);
        }
        // Only sample frames: a 3,000-tick run would otherwise write several
        // gigabytes of PNG and has no extra value over the trace.
        if index % every == 0
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
    }
    println!("frames_written={saved} last_index={last_serial}");
    std::fs::write(output.join("story-trace.txt"), trace.join("\n"))
        .map_err(|e| format!("write trace: {e}"))?;
    println!("debug={}", runtime.debug_info());
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for (name, _) in &requests {
        *counts.entry(name.as_str()).or_default() += 1;
    }
    println!("requests={:?}", counts);
    for (name, value) in requests.iter().filter(|(n, _)| n == "presentation_sprite").take(4) {
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
