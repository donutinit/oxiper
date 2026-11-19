//! The player (scene playlist + theme + options) and the interactive loop.

use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use crate::color::{Color, ColorMode};
use crate::config::{Background, Config, Override};
use crate::eggs::{self, Eggs};
use crate::fiesta::{Fiesta, Overlay};
use crate::render::{BOLD, Canvas, DIM, Renderer};
use crate::scene::{Ctx, Opts, Scene, SceneInfo};
use crate::scenes;
use crate::term::{self, Event, Key, Term};
use crate::theme::{self, TermColors, Theme};

/// Resolves a theme name: user themes first, then the dynamic and built-in ones.
pub fn resolve_theme(name: &str, cfg: &Config, tc: &TermColors) -> Result<Theme, String> {
    fn base(name: &str, tc: &TermColors) -> Result<Theme, String> {
        match name {
            "terminal" => Ok(Theme::terminal(tc)),
            "wal" | "pywal" => Theme::wal(),
            _ => Theme::builtin(name)
                .ok_or_else(|| format!("unknown theme '{name}' (see `oxiper list themes`)")),
        }
    }
    if let Some(t) = cfg.theme_table(name) {
        // `base` never looks at the user's themes, so `[themes.nord]` can
        // inherit the built-in nord it is named after.
        let inherit = t.get("inherit").and_then(|v| v.as_str());
        let parent = inherit.map(|p| base(p, tc)).transpose()?;
        return Theme::from_table(name, t, parent);
    }
    base(name, tc)
}

/// The theme a scene asks for in its own table (`theme = ""`, the default
/// the template shows, is none).
fn scene_theme(cfg: &Config, scene: &str) -> Option<String> {
    scene_str(cfg, scene, "theme").filter(|t| !t.is_empty())
}

/// The scenes whose own theme cannot be had. Any scene's, not only the
/// playlist's: the arrow keys reach them all.
fn scene_theme_problems(cfg: &Config, tc: &TermColors) -> Vec<String> {
    let problem = |info: &&SceneInfo| {
        let theme = scene_theme(cfg, info.name)?;
        let e = resolve_theme(&theme, cfg, tc).err()?;
        Some(format!("scenes.{}.theme: {e}", info.name))
    };
    scenes::ALL.iter().filter_map(problem).collect()
}

/// What `config check` has to say about the themes: the global one, the
/// user's own and each scene's.
pub fn theme_problems(cfg: &Config, tc: &TermColors) -> Vec<String> {
    let mut problems = Vec::new();
    if let Err(e) = resolve_theme(&cfg.theme, cfg, tc) {
        problems.push(e);
    }
    for n in cfg.custom_theme_names() {
        if let Err(e) = resolve_theme(&n, cfg, tc) {
            problems.push(e);
        }
    }
    problems.extend(scene_theme_problems(cfg, tc));
    problems
}

/// The hollywood preset's warnings, or why it cannot be used (spec 8.5, 8.6).
/// Checked whether or not hollywood is in the playlist: the arrow keys reach it.
pub fn preset_problems(cfg: &Config) -> Result<Vec<String>, String> {
    scenes::hollywood::preset_check(&cfg.scene_table("hollywood"))
}

/// Does this config need the terminal's 16-color palette?
pub fn wants_palette(cfg: &Config) -> bool {
    let uses = |n: &str| {
        n == "terminal"
            || cfg
                .theme_table(n)
                .and_then(|t| t.get("inherit"))
                .and_then(|v| v.as_str())
                == Some("terminal")
    };
    uses(&cfg.theme)
        || cfg.scenes.iter().any(|s| {
            cfg.scene_table(s.name)
                .get("theme")
                .and_then(|v| v.as_str())
                .is_some_and(uses)
        })
}

fn scene_str(cfg: &Config, scene: &str, key: &str) -> Option<String> {
    cfg.scene_table(scene)
        .get(key)
        .and_then(|v| v.as_str())
        .map(String::from)
}

fn scene_num(cfg: &Config, scene: &str, key: &str) -> Option<f64> {
    match cfg.scene_table(scene).get(key)? {
        toml::Value::Float(f) => Some(*f),
        toml::Value::Integer(i) => Some(*i as f64),
        _ => None,
    }
}

pub struct Player {
    pub cfg: Config,
    pub theme: Theme,
    pub tc: TermColors,
    pub bg: Color,
    order: Vec<usize>,
    pos: usize,
    pub info: &'static SceneInfo,
    opts: Opts,
    scene: Box<dyn Scene>,
    seed: u64,
    rng: fastrand::Rng,
    pub w: u16,
    pub h: u16,
    /// Seconds the current scene has been on screen (unscaled, pauses excluded).
    pub elapsed: f64,
    /// A theme picked interactively; wins over per-scene and global themes.
    forced_theme: Option<String>,
    /// The easter egg now playing, if any (see `crate::eggs`).
    eggs: Eggs,
    /// The holiday on (see `crate::fiesta`), what it puts over every scene,
    /// and real seconds since the date was last looked at.
    fiesta: Fiesta,
    overlay: Overlay,
    fiesta_age: f64,
}

/// Real seconds between looks at the date, so a holiday starts at midnight.
const FIESTA_CHECK: f64 = 60.0;

impl Player {
    pub fn new(cfg: Config, tc: TermColors, w: u16, h: u16) -> Result<Player, String> {
        // Validate the global theme even if the first scene brings its own,
        // and every scene's own, not only the first one's.
        let mut theme = resolve_theme(&cfg.theme, &cfg, &tc)?;
        if let Some(e) = scene_theme_problems(&cfg, &tc).into_iter().next() {
            return Err(e);
        }
        preset_problems(&cfg)?;
        let mut rng = cfg
            .seed
            .map_or_else(fastrand::Rng::new, fastrand::Rng::with_seed);
        let mut order: Vec<usize> = (0..cfg.scenes.len()).collect();
        if cfg.shuffle {
            rng.shuffle(&mut order);
        }
        let info = cfg.scenes[order[0]];
        if let Some(name) = scene_theme(&cfg, info.name) {
            theme = resolve_theme(&name, &cfg, &tc)?;
        }
        let seed = rng.u64(..);
        let opts = Opts::new(info.options, cfg.scene_table(info.name));
        let bg = Player::bg_for(&cfg, &theme, &tc);
        let fiesta = cfg.fiestas.resolve();
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg,
            seed,
            fiesta,
        };
        let scene = Player::make(info, &ctx, w, h);
        let mut eggs = Eggs::new(w, h);
        // Hung before the first frame, so the holiday is there from the start.
        let mut overlay = Overlay::new(w, h);
        overlay.update(0.0, fiesta);
        // A forced rate (for testing) plays an egg even in `snap` and `record`.
        if cfg.egg_rate >= eggs::FORCE_RATE {
            eggs.start(&cfg.eggs, fiesta);
        }
        Ok(Player {
            cfg,
            theme,
            tc,
            bg,
            order,
            pos: 0,
            info,
            opts,
            scene,
            seed,
            rng,
            w,
            h,
            elapsed: 0.0,
            forced_theme: None,
            eggs,
            fiesta,
            overlay,
            fiesta_age: 0.0,
        })
    }

    /// Seconds to show the current scene: its own `duration`, else the global one.
    pub fn duration(&self) -> f64 {
        scene_num(&self.cfg, self.info.name, "duration").unwrap_or(self.cfg.duration)
    }

    fn speed(&self) -> f32 {
        (self.cfg.speed * scene_num(&self.cfg, self.info.name, "pace").unwrap_or(1.0)) as f32
    }

    /// Switches to the theme the current scene wants, if it differs.
    fn apply_scene_theme(&mut self) {
        let want = self
            .forced_theme
            .clone()
            .or_else(|| scene_theme(&self.cfg, self.info.name))
            .unwrap_or_else(|| self.cfg.theme.clone());
        if want != self.theme.name
            && let Ok(t) = resolve_theme(&want, &self.cfg, &self.tc)
        {
            self.theme = t;
            self.bg = Player::bg_for(&self.cfg, &self.theme, &self.tc);
        }
    }

    /// The color fades should approach (see `Ctx::bg`).
    fn bg_for(cfg: &Config, theme: &Theme, tc: &TermColors) -> Color {
        match cfg.background {
            Background::Color(c) => return c,
            Background::Theme if theme.bg != Color::Reset => return theme.bg,
            _ => {}
        }
        cfg.bg_hint.or(tc.bg).unwrap_or(theme.bg)
    }

    /// The background painted into empty cells.
    pub fn canvas_bg(&self) -> Color {
        match self.cfg.background {
            Background::Transparent => Color::Reset,
            Background::Theme => self.theme.bg,
            Background::Color(c) => c,
        }
    }

    fn make(info: &SceneInfo, ctx: &Ctx, w: u16, h: u16) -> Box<dyn Scene> {
        let mut s = (info.make)(ctx);
        s.resize(ctx, w, h);
        s.retheme(ctx);
        s
    }

    fn rebuild(&mut self) {
        self.apply_scene_theme();
        self.opts = Opts::new(self.info.options, self.cfg.scene_table(self.info.name));
        let ctx = Ctx {
            theme: &self.theme,
            opts: &self.opts,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        };
        self.scene = Player::make(self.info, &ctx, self.w, self.h);
        self.elapsed = 0.0;
    }

    pub fn update(&mut self, dt: f32) {
        let ctx = Ctx {
            theme: &self.theme,
            opts: &self.opts,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        };
        // Scene time for motion, wall-clock time for the scene's clocks.
        self.scene.tick(&ctx, dt * self.speed(), dt);
        self.overlay.update(dt, self.fiesta);
        self.eggs.update(dt);
        if self.cfg.egg_rate >= eggs::FORCE_RATE {
            self.eggs.start(&self.cfg.eggs, self.fiesta);
        }
    }

    /// Looks at the date now and then, after `dt` real seconds, so a
    /// holiday starts at midnight even while a still scene sleeps (the
    /// interactive loop only: `snap`, `record` and `bench` stay reproducible).
    pub fn watch_calendar(&mut self, dt: f64) {
        self.fiesta_age += dt;
        if self.fiesta_age >= FIESTA_CHECK {
            self.fiesta_age = 0.0;
            self.set_fiesta(self.cfg.fiestas.resolve());
        }
    }

    /// Dresses the scene for a holiday (or for none) if it changed.
    fn set_fiesta(&mut self, fiesta: Fiesta) {
        if fiesta == self.fiesta {
            return;
        }
        self.fiesta = fiesta;
        self.overlay.update(0.0, fiesta);
        let ctx = Ctx {
            theme: &self.theme,
            opts: &self.opts,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        };
        self.scene.retheme(&ctx);
    }

    /// Lets the scene look at the files it reads besides its options (the
    /// interactive loop only, once a second: `snap`, `record` and `bench`
    /// stay reproducible). `Some`: a note or a reason for the HUD.
    pub fn refresh(&mut self) -> Option<Result<String, String>> {
        let ctx = Ctx {
            theme: &self.theme,
            opts: &self.opts,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        };
        self.scene.refresh(&ctx)
    }

    /// Rolls for an easter egg after `dt` real seconds (the interactive loop
    /// only: `snap`, `record` and `bench` stay reproducible).
    pub fn roll_egg(&mut self, dt: f64, allow: bool) {
        self.eggs
            .roll(&self.cfg.eggs, self.cfg.egg_rate, dt, allow, self.fiesta);
    }

    /// Plays an easter egg now (the hidden `E` key).
    pub fn start_egg(&mut self) {
        self.eggs.start(&self.cfg.eggs, self.fiesta);
    }

    pub fn draw(&mut self, c: &mut Canvas) {
        let ctx = Ctx {
            theme: &self.theme,
            opts: &self.opts,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        };
        self.scene.draw(&ctx, c);
        self.overlay.draw(c, &self.theme, self.fiesta);
        self.eggs.draw(c, &self.theme);
    }

    /// Real seconds until the scene's picture changes (see `Scene::next_change`).
    pub fn next_change(&self) -> f64 {
        if self.eggs.active() || self.fiesta.on() {
            return 0.0;
        }
        let speed = self.speed() as f64;
        (self.scene.next_change() as f64 / speed.max(1e-6)).max(0.0)
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.w = w;
        self.h = h;
        self.eggs.resize(w, h);
        self.overlay.resize(w, h);
        let ctx = Ctx {
            theme: &self.theme,
            opts: &self.opts,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        };
        self.scene.resize(&ctx, w, h);
    }

    /// Moves through the playlist (or through every scene when only one is selected).
    pub fn step_scene(&mut self, dir: i32) {
        if self.order.len() > 1 {
            let n = self.order.len() as i32;
            let next = (self.pos as i32 + dir).rem_euclid(n) as usize;
            if self.cfg.shuffle && dir > 0 && next == 0 {
                let current = self.order[self.pos];
                self.rng.shuffle(&mut self.order);
                if self.order[0] == current {
                    self.order.rotate_left(1);
                }
            }
            self.pos = next;
            self.info = self.cfg.scenes[self.order[self.pos]];
        } else {
            let all = scenes::ALL;
            let i = all
                .iter()
                .position(|s| s.name == self.info.name)
                .unwrap_or(0) as i32;
            self.info = all[(i + dir).rem_euclid(all.len() as i32) as usize];
        }
        self.seed = self.rng.u64(..);
        self.rebuild();
    }

    pub fn reseed(&mut self) {
        self.seed = self.rng.u64(..);
        self.rebuild();
    }

    pub fn theme_names(&self) -> Vec<String> {
        let mut v: Vec<String> = theme::names()
            .into_iter()
            .filter(|n| *n != "wal" || Theme::wal().is_ok())
            .map(String::from)
            .collect();
        v.extend(self.cfg.custom_theme_names());
        v
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.bg = Player::bg_for(&self.cfg, &self.theme, &self.tc);
        let ctx = Ctx {
            theme: &self.theme,
            opts: &self.opts,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        };
        self.scene.retheme(&ctx);
    }

    pub fn step_theme(&mut self, dir: i32) {
        let names = self.theme_names();
        let i = names
            .iter()
            .position(|n| *n == self.theme.name)
            .unwrap_or(0) as i32;
        let n = names.len() as i32;
        for k in 1..n {
            let name = &names[(i + dir * k).rem_euclid(n) as usize];
            if let Ok(t) = resolve_theme(name, &self.cfg, &self.tc) {
                self.forced_theme = Some(name.clone());
                self.set_theme(t);
                return;
            }
        }
    }

    /// Applies a reloaded config, keeping as much running state as possible.
    pub fn reload(&mut self, cfg: Config) -> Result<(), String> {
        let theme = resolve_theme(&cfg.theme, &cfg, &self.tc)?;
        if let Some(e) = scene_theme_problems(&cfg, &self.tc).into_iter().next() {
            return Err(e);
        }
        // For the HUD: the reason first, on one line.
        scenes::hollywood::preset_refusal(&cfg.scene_table("hollywood"))?;
        let same_list = cfg.scenes.iter().map(|s| s.name).eq(self
            .cfg
            .scenes
            .iter()
            .map(|s| s.name));
        let same_opts = cfg.scene_table(self.info.name) == self.cfg.scene_table(self.info.name);
        let same_bg = cfg.background == self.cfg.background;
        self.cfg = cfg;
        // Re-dressed below, with the theme.
        self.fiesta = self.cfg.fiestas.resolve();
        self.overlay.update(0.0, self.fiesta);
        if !same_list {
            self.order = (0..self.cfg.scenes.len()).collect();
            if self.cfg.shuffle {
                self.rng.shuffle(&mut self.order);
            }
            self.pos = 0;
            self.info = self.cfg.scenes[self.order[0]];
        }
        self.forced_theme = None;
        self.set_theme(theme);
        if !same_list || !same_opts || !same_bg {
            self.rebuild();
        } else {
            self.apply_scene_theme();
            let ctx = Ctx {
                theme: &self.theme,
                opts: &self.opts,
                bg: self.bg,
                seed: self.seed,
                fiesta: self.fiesta,
            };
            self.scene.retheme(&ctx);
        }
        Ok(())
    }

    fn label(&self) -> String {
        format!("{} · {}", self.info.name, self.theme.name)
    }
}

struct Hud {
    text: String,
    until: Instant,
    help: bool,
}

impl Hud {
    fn show(&mut self, text: String, secs: f32) {
        self.text = text;
        self.until = Instant::now() + Duration::from_secs_f32(secs);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let fg = if theme.fg == Color::Reset {
            Color::Reset
        } else {
            theme.fg
        };
        let accent = theme.pick(0, Color::Ansi(6));
        if Instant::now() < self.until && !self.text.is_empty() {
            let len = self.text.chars().count() as i32;
            let (x, y) = (c.w() as i32 - len - 2, c.h() as i32 - 1);
            c.text_attr(x - 1, y, " ", fg, 0);
            c.text_attr(x, y, &self.text, fg, BOLD);
            c.text_attr(x + len, y, " ", fg, 0);
        }
        if self.help {
            const LINES: &[(&str, &str)] = &[
                ("← →", "change scene"),
                ("↑ ↓", "change theme"),
                ("space", "pause"),
                ("+ -", "speed"),
                ("0", "reset speed"),
                ("r", "reseed scene"),
                ("?", "close help"),
                ("q", "quit"),
            ];
            let (bw, bh) = (30, LINES.len() as i32 + 4);
            let x0 = (c.w() as i32 - bw) / 2;
            let y0 = (c.h() as i32 - bh) / 2;
            for y in 0..bh {
                for x in 0..bw {
                    let ch = match (x, y) {
                        (0, 0) => '╭',
                        (x, 0) if x == bw - 1 => '╮',
                        (0, y) if y == bh - 1 => '╰',
                        (x, y) if x == bw - 1 && y == bh - 1 => '╯',
                        (_, 0) => '─',
                        (_, y) if y == bh - 1 => '─',
                        (0, _) => '│',
                        (x, _) if x == bw - 1 => '│',
                        _ => ' ',
                    };
                    c.set_attr(x0 + x, y0 + y, ch, accent, if ch == ' ' { 0 } else { DIM });
                }
            }
            c.text_attr(x0 + 2, y0, " oxiper ", accent, BOLD);
            for (i, (k, v)) in LINES.iter().enumerate() {
                let y = y0 + 2 + i as i32;
                c.text_attr(x0 + 3, y, k, accent, BOLD);
                c.text(x0 + 11, y, v, fg);
            }
        }
    }
}

pub struct RunOpts {
    pub config_path: Option<PathBuf>,
    pub overrides: Vec<Override>,
    pub saver: bool,
}

/// What a reload of `config.toml` changed that the loop has to act on.
pub struct Reloaded {
    pub bg_changed: bool,
    pub mode: ColorMode,
}

/// The once-a-second look at config.toml. None: unchanged. A refusal records
/// the file's mtime, so it is not tried again until the file changes (K9).
fn poll_config(player: &mut Player, ro: &RunOpts) -> Option<Result<Reloaded, String>> {
    if !player.cfg.changed_on_disk() {
        return None;
    }
    let refused = match Config::load(ro.config_path.as_deref(), &ro.overrides, ro.saver) {
        Ok(cfg) => {
            let bg_changed =
                cfg.background != player.cfg.background || cfg.strict != player.cfg.strict;
            let mode = cfg.color.unwrap_or_else(ColorMode::detect);
            match player.reload(cfg) {
                Ok(()) => return Some(Ok(Reloaded { bg_changed, mode })),
                Err(e) => e,
            }
        }
        Err(e) => e,
    };
    // Keep the old config but remember the mtime so we don't retry every second.
    player.cfg.mtime = player
        .cfg
        .path
        .as_ref()
        .and_then(|p| std::fs::metadata(p).and_then(|m| m.modified()).ok());
    Some(Err(refused))
}

pub fn run(cfg: Config, ro: RunOpts) -> Result<(), String> {
    term::install_handlers();
    let setup = |e: std::io::Error| format!("cannot set up the terminal: {e}");
    let mut term = Term::open().map_err(setup)?;
    term.enter(ro.saver && cfg.exit_on_mouse).map_err(setup)?;
    let tc = term.query_colors(wants_palette(&cfg));
    let mode = cfg.color.unwrap_or_else(ColorMode::detect);
    let (w, h) = term.size();
    let hud_on = cfg.hud;
    let mut player = Player::new(cfg, tc, w, h)?;
    let mut canvas = Canvas::new(w, h, player.canvas_bg());
    canvas.strict = player.cfg.strict;
    let mut renderer = Renderer::new(mode);
    let mut hud = Hud {
        text: String::new(),
        until: Instant::now(),
        help: false,
    };
    let mut events = Vec::with_capacity(16);
    let started = Instant::now();
    let mut last = Instant::now();
    let mut last_check = Instant::now();
    let mut paused = false;
    let mut focused = true;
    let mut dirty = true;
    let quit = || term::QUIT.store(true, Ordering::SeqCst);

    while !term::QUIT.load(Ordering::SeqCst) {
        let frame_start = Instant::now();
        let dt = (frame_start - last).as_secs_f32().min(0.1);
        let real_dt = (frame_start - last).as_secs_f64();
        last = frame_start;
        let fps = if focused {
            player.cfg.fps
        } else {
            player.cfg.unfocused_fps
        };
        let running = !paused && fps > 0;

        if running {
            player.update(dt);
            player.roll_egg(real_dt, !hud.help);
            player.watch_calendar(real_dt);
            player.elapsed += dt as f64;
            dirty = true;
            if player.duration() > 0.0
                && player.cfg.scenes.len() > 1
                && player.elapsed >= player.duration()
            {
                player.step_scene(1);
                if hud_on && !ro.saver {
                    hud.show(player.label(), 2.0);
                }
            }
        }
        let hud_visible = Instant::now() < hud.until || hud.help;
        if dirty || hud_visible {
            canvas.clear();
            player.draw(&mut canvas);
            hud.draw(&mut canvas, &player.theme);
            let bytes = renderer.frame(&canvas);
            if !bytes.is_empty() && term.out.write_all(bytes).is_err() {
                break;
            }
            dirty = false;
        }

        if last_check.elapsed() >= Duration::from_secs(1) {
            last_check = Instant::now();
            if let Some(polled) = poll_config(&mut player, &ro) {
                match polled {
                    Ok(r) => {
                        if r.bg_changed || r.mode != renderer.mode {
                            renderer.mode = r.mode;
                            canvas.set_background(player.canvas_bg());
                            canvas.strict = player.cfg.strict;
                            renderer.invalidate();
                        }
                        hud.show("config reloaded".into(), 1.5);
                    }
                    Err(e) => hud.show(e, 5.0),
                }
                dirty = true;
            }
            // The scene's own file (hollywood's preset), paused or not.
            if let Some(refreshed) = player.refresh() {
                match refreshed {
                    Ok(note) => hud.show(note, 2.0),
                    Err(why) => hud.show(why, 5.0),
                }
                dirty = true;
            }
        }

        // Sleep until the next frame, or longer when the scene says nothing will
        // change before then; input and signals wake us early either way.
        let deadline = if running {
            let mut wait = (1.0 / fps as f64).max(player.next_change().min(1.0));
            if player.duration() > 0.0 && player.cfg.scenes.len() > 1 {
                wait = wait.min((player.duration() - player.elapsed).max(0.0) + 0.001);
            }
            if hud_visible {
                wait = wait.min(0.25);
            }
            frame_start + Duration::from_secs_f64(wait)
        } else {
            Instant::now() + Duration::from_millis(250)
        };
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() || term::QUIT.load(Ordering::SeqCst) {
                break;
            }
            events.clear();
            term.poll(left, &mut events);
            // Ignore keys that arrive right at start-up (the keystroke that
            // launched us, auto-repeat).
            let settled = started.elapsed() > Duration::from_millis(300);
            for &ev in &events {
                match ev {
                    Event::Hangup => quit(),
                    Event::Resize => {
                        let (w, h) = term.size();
                        canvas.resize(w, h);
                        renderer.invalidate();
                        player.resize(w, h);
                        dirty = true;
                    }
                    Event::FocusOut => focused = false,
                    Event::FocusIn => {
                        focused = true;
                        last = Instant::now();
                    }
                    Event::MouseDown if ro.saver && settled => quit(),
                    Event::Key(_) if ro.saver && settled => quit(),
                    Event::Key(k) if settled && !ro.saver => {
                        match k {
                            Key::CtrlC | Key::CtrlD | Key::Esc | Key::Char('q') => quit(),
                            Key::Right | Key::Char('l' | 'n') => {
                                player.step_scene(1);
                                hud.show(player.label(), 1.6);
                            }
                            Key::Left | Key::Char('h' | 'p') => {
                                player.step_scene(-1);
                                hud.show(player.label(), 1.6);
                            }
                            Key::Up | Key::Char('k' | 't') => {
                                player.step_theme(1);
                                hud.show(player.label(), 1.6);
                            }
                            Key::Down | Key::Char('j' | 'T') => {
                                player.step_theme(-1);
                                hud.show(player.label(), 1.6);
                            }
                            Key::Char(' ') => {
                                paused = !paused;
                                hud.show(
                                    if paused {
                                        "paused".into()
                                    } else {
                                        player.label()
                                    },
                                    1.2,
                                );
                                last = Instant::now();
                            }
                            Key::Char('+' | '=') => {
                                player.cfg.speed = (player.cfg.speed * 1.25).min(50.0);
                                hud.show(format!("speed {:.2}×", player.cfg.speed), 1.2);
                            }
                            Key::Char('-' | '_') => {
                                player.cfg.speed = (player.cfg.speed / 1.25).max(0.05);
                                hud.show(format!("speed {:.2}×", player.cfg.speed), 1.2);
                            }
                            Key::Char('0') => {
                                player.cfg.speed = 1.0;
                                hud.show("speed 1×".into(), 1.2);
                            }
                            Key::Char('r') => {
                                player.reseed();
                                hud.show(format!("{} · reseeded", player.info.name), 1.2);
                            }
                            Key::Char('?') => hud.help = !hud.help,
                            Key::Char('E') if !hud.help => player.start_egg(),
                            _ => {}
                        }
                        dirty = true;
                    }
                    _ => {}
                }
            }
        }
    }
    drop(term);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Kind;

    /// The first day, the height and the last day of every holiday in a
    /// year, and the Navidad's own days.
    fn fiesta_days() -> Vec<Fiesta> {
        let mut days: Vec<Fiesta> = Vec::new();
        let mut add = |f: Fiesta| {
            if f.on() && !days.contains(&f) {
                days.push(f);
            }
        };
        for kind in Kind::ALL {
            add(Fiesta::height(kind));
            let mut seen: Vec<Fiesta> = Vec::new();
            for month in 1..=12 {
                let last = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][month as usize - 1];
                for day in 1..=last {
                    let f = Fiesta::of_date(2027, month, day);
                    if f.is(kind) {
                        seen.push(f);
                    }
                }
            }
            if let (Some(&first), Some(&last)) = (seen.first(), seen.last()) {
                add(first);
                add(last);
            }
        }
        for (month, day) in [
            (12, 16),
            (12, 23),
            (12, 24),
            (12, 28),
            (12, 31),
            (1, 1),
            (1, 6),
        ] {
            add(Fiesta::of_date(2027, month, day));
        }
        days
    }

    /// `[themes.nord]` with `inherit = "nord"` is the built-in nord with
    /// the user's changes, not a theme built on nothing.
    #[test]
    fn a_theme_can_inherit_the_builtin_it_is_named_after() {
        let cfg = Config::from_toml("[themes.nord]\ninherit = 'nord'\nfg = '#ff0000'\n");
        let t = resolve_theme("nord", &cfg, &TermColors::default()).unwrap();
        let nord = Theme::builtin("nord").unwrap();
        assert_eq!(t.fg, Color::Rgb(255, 0, 0));
        assert_eq!((t.ramp, t.colors, t.bg), (nord.ramp, nord.colors, nord.bg));
    }

    fn player(toml: &str) -> Result<Player, String> {
        Player::new(Config::from_toml(toml), TermColors::default(), 80, 24)
    }

    /// A scene's own theme is checked like the global one, wherever the
    /// scene is in the playlist (it used to be an error for the first scene
    /// and silently ignored for the others).
    #[test]
    fn a_scenes_unknown_theme_is_an_error() {
        for order in ["fire,neo", "neo,fire", "neo"] {
            let toml =
                format!("scene = '{order}'\nfiestas = 'none'\n[scenes.fire]\ntheme = 'nope'\n");
            let Err(e) = player(&toml) else {
                panic!("{order}: started with an unknown theme");
            };
            assert!(e.contains("scenes.fire.theme") && e.contains("nope"), "{e}");
        }
        // A reload with one is refused, and the old config stays.
        let mut p = player("scene = 'neo,fire'\nfiestas = 'none'\n")
            .ok()
            .unwrap();
        let bad =
            Config::from_toml("scene = 'neo,fire'\nfps = 12\n[scenes.fire]\ntheme = 'nope'\n");
        assert!(p.reload(bad).is_err());
        assert_eq!(p.cfg.fps, 30);
        // `theme = ""`, the default the template shows, is no theme of its own.
        let p =
            player("scene = 'fire'\nfiestas = 'none'\ntheme = 'nord'\n[scenes.fire]\ntheme = ''\n");
        assert_eq!(p.ok().unwrap().theme.name, "nord");
    }

    /// `config check` reports a scene's unknown theme too.
    #[test]
    fn config_check_sees_scene_themes() {
        let tc = TermColors::default();
        let cfg =
            Config::from_toml("[scenes.fire]\ntheme = 'nope'\n[scenes.neo]\ntheme = 'nord'\n");
        let problems = theme_problems(&cfg, &tc);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("scenes.fire.theme"), "{problems:?}");
        assert!(theme_problems(&Config::from_toml("theme = 'nord'"), &tc).is_empty());
        assert_eq!(
            theme_problems(&Config::from_toml("theme = 'nope'"), &tc).len(),
            1
        );
    }

    /// A file of this test's own under the temporary directory, holding `text`.
    fn temp_file(name: &str, text: &str) -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("oxiper-app-{}-{name}.toml", std::process::id()));
        std::fs::write(&path, text).unwrap();
        path
    }

    /// `[scenes.hollywood]` naming the preset at `path`.
    fn naming(path: &std::path::Path) -> String {
        format!("[scenes.hollywood]\npreset = '{}'\n", path.display())
    }

    /// A preset that cannot be used and has never loaded stops oxiper before
    /// it starts, whether or not hollywood is in the playlist (spec 8.6).
    #[test]
    fn a_broken_preset_stops_the_start() {
        let broken = temp_file("start-broken", "tema = 'software libre'\ntitulos = [\n");
        let missing = std::env::temp_dir().join(format!(
            "oxiper-app-{}-start-missing.toml",
            std::process::id()
        ));
        for scene in ["hollywood", "neo"] {
            let head = format!("scene = '{scene}'\nfiestas = 'none'\n");
            let Err(e) = player(&format!("{head}{}", naming(&broken))) else {
                panic!("{scene}: started with a broken preset");
            };
            assert!(e.contains(&*broken.display().to_string()), "{e}");
            assert!(e.contains("line"), "{e}");
            let Err(e) = player(&format!("{head}{}", naming(&missing))) else {
                panic!("{scene}: started with a missing preset");
            };
            assert!(e.contains(&*missing.display().to_string()), "{e}");
            let Err(e) = player(&format!("{head}[scenes.hollywood]\npreset = 2026\n")) else {
                panic!("{scene}: started with a preset that is a number");
            };
            assert!(e.contains("quote"), "{e}");
        }
    }

    /// An empty file is a valid preset, the neutral base, when nothing
    /// loaded from it before.
    #[test]
    fn an_empty_preset_that_never_loaded_is_the_base() {
        let empty = temp_file("start-empty", "");
        let toml = format!("scene = 'hollywood'\nfiestas = 'none'\n{}", naming(&empty));
        assert!(player(&toml).is_ok());
    }

    /// The bottom row of the player's next frame.
    fn bottom_row(p: &mut Player) -> String {
        let mut c = Canvas::new(p.w, p.h, Color::Reset);
        p.draw(&mut c);
        let w = p.w as usize;
        c.cells()[c.cells().len() - w..]
            .iter()
            .map(|cell| cell.ch)
            .collect()
    }

    /// A reload that names a preset that never loaded and is broken is
    /// refused: the scene, the config in use and the old preset stay
    /// (spec 11.4).
    #[test]
    fn a_reload_naming_a_broken_preset_is_refused() {
        let good = temp_file("reload-good", "titulares = ['Lo que se queda']\n");
        let broken = temp_file("reload-broken", "tema = \n");
        let mut p = player(&format!(
            "scene = 'hollywood,neo'\nfiestas = 'none'\n{}",
            naming(&good)
        ))
        .ok()
        .unwrap();
        assert!(bottom_row(&mut p).contains("LO QUE SE QUEDA"));
        // Which scene is on: the same one, not one built again.
        let on = |p: &Player| &*p.scene as *const dyn Scene as *const ();
        let scene = on(&p);
        let bad = Config::from_toml(&format!(
            "scene = 'hollywood,neo'\nfps = 12\n{}",
            naming(&broken)
        ));
        let Err(e) = p.reload(bad) else {
            panic!("a reload naming a broken preset was taken");
        };
        assert!(e.starts_with("preset: ") && e.chars().count() <= 60, "{e}");
        assert_eq!(p.cfg.fps, 30);
        assert_eq!(p.info.name, "hollywood");
        assert!(std::ptr::eq(on(&p), scene));
        p.update(5.0);
        assert!(bottom_row(&mut p).contains("LO QUE SE QUEDA"));
        assert_eq!(p.refresh(), None);
    }

    /// The interactive loop's once-a-second look reaches the scene: an edit of
    /// hollywood's preset is taken in; a scene with no file answers nothing.
    #[test]
    fn refresh_reaches_the_scene() {
        let path = temp_file("refresh", "titulares = ['Antes']\n");
        let mut p = player(&format!(
            "scene = 'hollywood'\nfiestas = 'none'\n{}",
            naming(&path)
        ))
        .ok()
        .unwrap();
        assert_eq!(p.refresh(), None);
        std::fs::write(&path, "titulares = ['Despues de editar']\n").unwrap();
        assert!(matches!(p.refresh(), Some(Ok(_))));
        assert!(bottom_row(&mut p).contains("DESPUES DE EDITAR"));
        assert_eq!(p.refresh(), None);
        let mut neo = player("scene = 'neo'\nfiestas = 'none'\n").ok().unwrap();
        assert_eq!(neo.refresh(), None);
    }

    /// `config check` lists a preset's warnings, each with the preset's
    /// name, and its error.
    #[test]
    fn config_check_sees_presets() {
        let one_bad = temp_file(
            "check-one-bad",
            "titulares = ['GNU no es Unix', 'Hola {n']\n",
        );
        let stem = one_bad.file_stem().unwrap().to_str().unwrap();
        let cfg = Config::from_toml(&naming(&one_bad));
        let problems = preset_problems(&cfg).unwrap();
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].starts_with(&format!("{stem}: ")),
            "{problems:?}"
        );
        assert!(problems[0].contains("titulares["), "{problems:?}");
        let broken = temp_file("check-broken", "titulares = [\n");
        assert!(preset_problems(&Config::from_toml(&naming(&broken))).is_err());
        assert_eq!(preset_problems(&Config::from_toml("")), Ok(vec![]));
    }

    /// A reload that the `Player` refuses is not tried again until the file
    /// changes: two polls after one bad save make one error (K9).
    #[test]
    fn a_refused_reload_is_tried_once() {
        let path = temp_file("poll-config", "scene = 'neo'\nfiestas = 'none'\n");
        let cfg = Config::load(Some(&path), &[], false).unwrap();
        let mut p = Player::new(cfg, TermColors::default(), 80, 24)
            .ok()
            .unwrap();
        let ro = RunOpts {
            config_path: Some(path.clone()),
            overrides: vec![],
            saver: false,
        };
        assert!(poll_config(&mut p, &ro).is_none());
        // The mtime alone can be the same within one tick: set it.
        let rewrite = |text: &str, secs: u64| {
            std::fs::write(&path, text).unwrap();
            let f = std::fs::File::options().write(true).open(&path).unwrap();
            f.set_modified(std::time::SystemTime::UNIX_EPOCH + Duration::from_secs(secs))
                .unwrap();
        };
        rewrite(
            "scene = 'neo'\nfiestas = 'none'\ntheme = 'nope'\n",
            1_000_000,
        );
        assert!(matches!(poll_config(&mut p, &ro), Some(Err(_))));
        assert!(poll_config(&mut p, &ro).is_none());
        rewrite("scene = 'neo'\nfiestas = 'none'\nfps = 12\n", 2_000_000);
        assert!(matches!(poll_config(&mut p, &ro), Some(Ok(_))));
        assert_eq!(p.cfg.fps, 12);
        assert!(poll_config(&mut p, &ro).is_none());
    }

    /// Every scene survives every holiday's days at odd sizes, with the
    /// holiday changing under it (at midnight, on a config reload) and
    /// ending, and the overlay drawn on top.
    #[test]
    fn scenes_survive_every_fiesta() {
        let days = fiesta_days();
        assert!(days.len() > Kind::ALL.len());
        // One thread per scene: this is the slowest test.
        std::thread::scope(|s| {
            for info in scenes::ALL {
                let days = &days;
                s.spawn(move || {
                    let cfg = Config::from_toml(&format!(
                        "scene = '{}'\nseed = 7\nfiestas = 'none'\n",
                        info.name
                    ));
                    let mut p = Player::new(cfg, TermColors::default(), 80, 24).unwrap();
                    for &fiesta in days.iter().chain([&Fiesta::NONE]) {
                        p.set_fiesta(fiesta);
                        for &(w, h) in &[(80u16, 24u16), (1, 1), (2, 3), (7, 2), (13, 5), (300, 90)]
                        {
                            p.resize(w, h);
                            let mut c = Canvas::new(w, h, Color::Reset);
                            for i in 0..32 {
                                p.update(if i < 30 { 1.0 / 30.0 } else { 5.0 });
                                c.clear();
                                p.draw(&mut c);
                            }
                        }
                    }
                });
            }
        });
    }

    /// Outside set mode `r` and the arrow keys draw the seeds they always drew.
    #[test]
    fn reseeding_draws_the_seeds_it_always_drew() {
        for (shuffle, want) in [
            (
                false,
                [
                    14266376187910451857u64,
                    4104141878723513376,
                    13250119348504322419,
                    8875224889773325713,
                    2148687651783032108,
                ],
            ),
            (
                true,
                [
                    13250119348504322419,
                    8875224889773325713,
                    2148687651783032108,
                    3053616117984519433,
                    14602987178384512605,
                ],
            ),
        ] {
            let mut p = player(&format!(
                "scene = 'neo,fire,rain'\nseed = 7\nshuffle = {shuffle}\nfiestas = 'none'\n"
            ))
            .ok()
            .unwrap();
            let mut got = vec![p.seed];
            p.reseed();
            got.push(p.seed);
            p.step_scene(1);
            got.push(p.seed);
            p.step_scene(-1);
            got.push(p.seed);
            p.reseed();
            got.push(p.seed);
            assert_eq!(got, want, "shuffle {shuffle}");
        }
    }
}
