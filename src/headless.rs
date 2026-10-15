//! Running scenes without a terminal: asciicast recording, snapshots and benchmarks.

use std::io::Write;
use std::time::Instant;

use crate::app::Player;
use crate::color::{Color, ColorMode};
use crate::config::Config;
use crate::render::{BOLD, Canvas, DIM, Renderer};
use crate::theme::TermColors;

pub fn parse_size(s: &str) -> Result<(u16, u16), String> {
    let (w, h) = s
        .split_once(['x', 'X'])
        .ok_or(format!("size should look like 80x24, got '{s}'"))?;
    let w: u16 = w
        .trim()
        .parse()
        .map_err(|_| format!("bad width in '{s}'"))?;
    let h: u16 = h
        .trim()
        .parse()
        .map_err(|_| format!("bad height in '{s}'"))?;
    if w == 0 || h == 0 {
        return Err("size must be at least 1x1".into());
    }
    Ok((w, h))
}

fn player(cfg: Config, w: u16, h: u16) -> Result<(Player, Canvas), String> {
    let p = Player::new(cfg, TermColors::default(), w, h)?;
    let mut c = Canvas::new(w, h, p.canvas_bg());
    c.strict = p.cfg.strict;
    Ok((p, c))
}

fn json_escape(bytes: &[u8], out: &mut String) {
    for ch in String::from_utf8_lossy(bytes).chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 || c == '\u{7f}' => {
                out.push_str(&format!("\\u{:04x}", c as u32))
            }
            c => out.push(c),
        }
    }
}

/// Writes an asciicast v2 recording (playable with `asciinema play`, convertible with `agg`).
pub fn record(
    cfg: Config,
    w: u16,
    h: u16,
    seconds: f64,
    skip: f64,
    fps: u32,
    out: &mut dyn Write,
) -> Result<(), String> {
    let mode = cfg.color.unwrap_or(ColorMode::TrueColor);
    let (mut p, mut canvas) = player(cfg, w, h)?;
    let mut r = Renderer::new(mode);
    r.sync = false;
    let title = p.info.name;
    let io = |e: std::io::Error| e.to_string();
    writeln!(out, "{{\"version\": 2, \"width\": {w}, \"height\": {h}, \"title\": \"oxiper {title}\", \"env\": {{\"TERM\": \"xterm-256color\"}}}}").map_err(io)?;
    let dt = 1.0 / fps.max(1) as f32;
    let frames = (seconds * fps as f64).round() as usize;
    let mut line = String::new();
    for _ in 0..(skip * fps as f64) as usize {
        p.update(dt);
    }
    let cycle = p.cfg.scenes.len() > 1;
    for i in 0..frames {
        if i > 0 {
            p.update(dt);
            p.elapsed += dt as f64;
            if cycle && p.duration() > 0.0 && p.elapsed >= p.duration() {
                p.step_scene(1);
            }
        }
        canvas.clear();
        p.draw(&mut canvas);
        let mut bytes = Vec::new();
        if i == 0 {
            bytes.extend_from_slice(b"\x1b[?25l\x1b[?7l");
        }
        bytes.extend_from_slice(r.frame(&canvas));
        if bytes.is_empty() {
            continue;
        }
        line.clear();
        line.push_str(&format!("[{:.4}, \"o\", \"", i as f64 / fps as f64));
        json_escape(&bytes, &mut line);
        line.push_str("\"]");
        writeln!(out, "{line}").map_err(io)?;
    }
    Ok(())
}

/// Renders the scene `at` seconds in and returns it as text (or with colors).
pub fn snap(cfg: Config, w: u16, h: u16, at: f64, ansi: bool) -> Result<String, String> {
    let (mut p, mut canvas) = player(cfg, w, h)?;
    let dt = 1.0 / 30.0;
    let mut t = 0.0;
    while t < at {
        p.update(dt as f32);
        t += dt;
    }
    canvas.clear();
    p.draw(&mut canvas);
    if !ansi {
        return Ok(canvas.to_text());
    }
    let sgr = |c: Color, bg: bool| -> String {
        match c {
            Color::Reset => String::new(),
            Color::Ansi(n) => format!("\x1b[{};5;{n}m", if bg { 48 } else { 38 }),
            Color::Rgb(r, g, b) => format!("\x1b[{};2;{r};{g};{b}m", if bg { 48 } else { 38 }),
        }
    };
    let mut s = String::new();
    for y in 0..h as i32 {
        let mut cur = None;
        for x in 0..w as i32 {
            let c = *canvas.get(x, y).unwrap();
            if cur != Some((c.fg, c.bg, c.attr)) {
                s.push_str("\x1b[0m");
                if c.attr & BOLD != 0 {
                    s.push_str("\x1b[1m");
                }
                if c.attr & DIM != 0 {
                    s.push_str("\x1b[2m");
                }
                s.push_str(&sgr(c.fg, false));
                s.push_str(&sgr(c.bg, true));
                cur = Some((c.fg, c.bg, c.attr));
            }
            s.push(c.ch);
        }
        s.push_str("\x1b[0m\n");
    }
    Ok(s)
}

pub struct BenchRow {
    pub name: &'static str,
    pub sim_us: f64,
    pub render_us: f64,
    pub bytes: f64,
    pub changed: f64,
}

/// Times `frames` frames of each scene: simulation+drawing, then diffing+encoding.
pub fn bench(
    mk: &dyn Fn(&'static str) -> Result<Config, String>,
    names: &[&'static str],
    w: u16,
    h: u16,
    frames: usize,
) -> Result<Vec<BenchRow>, String> {
    let mut rows = Vec::new();
    for &name in names {
        let cfg = mk(name)?;
        let (mut p, mut canvas) = player(cfg, w, h)?;
        let mut r = Renderer::new(ColorMode::TrueColor);
        let dt = 1.0 / 30.0;
        // Warm up so scenes that start empty are measured in their steady state.
        for _ in 0..90 {
            p.update(dt);
            canvas.clear();
            p.draw(&mut canvas);
            r.frame(&canvas);
        }
        let (mut sim, mut ren, mut bytes, mut changed) = (0.0, 0.0, 0usize, 0usize);
        let mut prev = canvas.cells().to_vec();
        for _ in 0..frames {
            let t0 = Instant::now();
            p.update(dt);
            canvas.clear();
            p.draw(&mut canvas);
            let t1 = Instant::now();
            let n = r.frame(&canvas).len();
            let t2 = Instant::now();
            sim += (t1 - t0).as_secs_f64();
            ren += (t2 - t1).as_secs_f64();
            bytes += n;
            changed += canvas
                .cells()
                .iter()
                .zip(&prev)
                .filter(|(a, b)| a != b)
                .count();
            prev.copy_from_slice(canvas.cells());
        }
        let f = frames as f64;
        rows.push(BenchRow {
            name,
            sim_us: sim / f * 1e6,
            render_us: ren / f * 1e6,
            bytes: bytes as f64 / f,
            changed: changed as f64 / f / (w as f64 * h as f64) * 100.0,
        });
    }
    Ok(rows)
}
