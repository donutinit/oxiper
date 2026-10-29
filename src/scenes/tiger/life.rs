//! Everything else alive in the jungle, and what flies about: herds that
//! graze until they notice the tiger and bolt, langurs barking alarms from
//! the trees, birds that flush, the rivals that come now and then to try
//! the tiger (and always leave faster than they came), and specks of dust,
//! spray and falling leaves.
//!
//! The jungle has no end, so its animals are dealt out around the camera:
//! herds come in from beyond the edges of the view while there are fewer
//! than the `prey` option asks for, and go once they are far away.

use super::art::{self, Palette, Sprite};
use super::body::{Build, Gait, Legs, REAR, TailPose};
use super::hunter::Mood;
use super::jungle::{FEET, Jungle};
use super::{Dice, Rgb, Rng, unit};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Species {
    Chital,
    Sambar,
    RedDeer,
    Boar,
    Peacock,
}

impl Species {
    fn palette(self) -> Palette {
        match self {
            Species::Chital => art::CHITAL_COLORS,
            Species::Sambar => art::SAMBAR_COLORS,
            Species::RedDeer => art::RED_DEER_COLORS,
            Species::Boar => art::BOAR_COLORS,
            Species::Peacock => art::PEACOCK_COLORS,
        }
    }

    /// How fast it bolts, in world pixels per step.
    fn flight(self) -> f32 {
        match self {
            Species::Chital => 2.8,
            Species::Sambar => 2.4,
            Species::RedDeer => 2.7,
            Species::Boar => 2.3,
            Species::Peacock => 1.6,
        }
    }

    /// How many make a herd.
    fn herd(self, rng: &mut Rng) -> i32 {
        match self {
            Species::Chital => rng.range(3, 7),
            Species::Sambar => rng.range(1, 3),
            Species::RedDeer => rng.range(2, 5),
            Species::Boar => rng.range(2, 5),
            Species::Peacock => rng.range(1, 3),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Graze,
    Walk,
    /// Head up, staring, about to run.
    Alert,
    Flee,
}

#[derive(Clone, Copy, Debug)]
pub struct Beast {
    pub id: u32,
    pub species: Species,
    pub x: f32,
    pub vx: f32,
    pub dir: f32,
    pub state: State,
    steps: u32,
    /// Steps it stays in this state.
    wait: u32,
    phase: f32,
    /// Antlers (or a peacock's train).
    stag: bool,
    /// Off the ground: a peacock flying off.
    lift: f32,
    herd: u32,
}

/// What the tiger may know of a beast.
#[derive(Clone, Copy, Debug)]
pub struct Sighting {
    pub x: f32,
    pub vx: f32,
    pub fleeing: bool,
}

/// A langur on a branch.
#[derive(Clone, Copy)]
struct Langur {
    x: f32,
    y: f32,
    dir: f32,
    /// Steps left barking.
    bark: u32,
    phase: f32,
    tree: i64,
}

/// A small bird, on the ground or in the air.
#[derive(Clone, Copy)]
struct Bird {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    flying: bool,
    kind: usize,
    phase: f32,
    dir: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RivalKind {
    /// A sloth bear, or a brown bear in the taiga.
    Bear,
    /// One of a pack of dholes, or of wolves.
    Dog,
    Croc,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RivalState {
    /// Coming at the tiger.
    Come,
    /// Squaring up: the bear rears, the dogs bay.
    Face,
    /// Running for it.
    Flee,
    /// A crocodile lying in wait, eyes above the water.
    Lurk,
    /// The crocodile's lunge.
    Lunge,
    /// Gone under.
    Dive,
}

#[derive(Clone, Copy, Debug)]
pub struct Rival {
    pub id: u32,
    pub kind: RivalKind,
    pub x: f32,
    pub dir: f32,
    pub state: RivalState,
    steps: u32,
    phase: f32,
    speed: f32,
    /// Its place in the pack, so dogs keep their distances.
    slot: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Fleck {
    Dust,
    Spray,
    Leaf,
    /// A petal: marigold on Día de Muertos, nochebuena at Navidad.
    Petal,
}

#[derive(Clone, Copy)]
struct Speck {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: u16,
    span: u16,
    kind: Fleck,
    shade: u8,
}

#[derive(Default)]
pub struct Life {
    pub beasts: Vec<Beast>,
    langurs: Vec<Langur>,
    /// Trees whose langurs are about.
    langur_trees: Vec<i64>,
    birds: Vec<Bird>,
    pub rivals: Vec<Rival>,
    specks: Vec<Speck>,
    next_id: u32,
    /// Real seconds until a rival comes to try the tiger.
    rival_clock: f32,
    /// Rivers whose crocodile has already had its go.
    croc_done: Vec<i64>,
    /// How many beasts have bolted, and been caught.
    pub bolted: u32,
    pub eaten: u32,
    /// How many rivals have been seen off.
    pub routed: u32,
}

impl Life {
    pub fn clocks(&mut self, real: f32) {
        self.rival_clock -= real;
    }

    pub fn shift(&mut self, dx: f32) {
        for b in &mut self.beasts {
            b.x -= dx;
        }
        for l in &mut self.langurs {
            l.x -= dx;
        }
        for b in &mut self.birds {
            b.x -= dx;
        }
        for r in &mut self.rivals {
            r.x -= dx;
        }
        for s in &mut self.specks {
            s.x -= dx;
        }
    }

    fn id(&mut self) -> u32 {
        self.next_id = self.next_id.wrapping_add(1);
        self.next_id
    }

    /// The nearest beast within `range` of `x` that is not already running.
    pub fn prey_near(&self, x: f32, range: f32) -> Option<u32> {
        self.beasts
            .iter()
            .filter(|b| b.state != State::Flee && b.lift == 0.0 && (b.x - x).abs() < range)
            .min_by(|a, b| (a.x - x).abs().total_cmp(&(b.x - x).abs()))
            .map(|b| b.id)
    }

    pub fn beast(&self, id: u32) -> Option<Sighting> {
        self.beasts.iter().find(|b| b.id == id).map(|b| Sighting {
            x: b.x,
            vx: b.vx,
            fleeing: b.state == State::Flee,
        })
    }

    /// Caught: gone in a cloud of dust, and the rest of its herd runs.
    pub fn catch(&mut self, id: u32, rng: &mut Rng) {
        if let Some(i) = self.beasts.iter().position(|b| b.id == id) {
            let b = self.beasts.remove(i);
            self.eaten += 1;
            self.dust(b.x, FEET as f32 - 2.0, 14, rng);
            self.scare(b.x, 160.0, false);
        }
    }

    /// Something frightening at `x`: beasts within `range` bolt, birds go
    /// up, langurs bark. A roar reaches further and flushes every bird.
    pub fn scare(&mut self, x: f32, range: f32, roar: bool) {
        for b in &mut self.beasts {
            if (b.x - x).abs() < range && b.state != State::Flee {
                b.state = State::Flee;
                b.dir = if b.x >= x { 1.0 } else { -1.0 };
                b.steps = 0;
                self.bolted += 1;
            }
        }
        for b in &mut self.birds {
            if !b.flying && (roar || (b.x - x).abs() < range * 0.6) && (b.x - x).abs() < range {
                flush(b, x);
            }
        }
        for l in &mut self.langurs {
            if (l.x - x).abs() < range {
                l.bark = l.bark.max(70);
            }
        }
    }

    /// A rival close enough to deal with.
    pub fn rival_near(&self, x: f32, range: f32) -> Option<u32> {
        self.rivals
            .iter()
            .find(|r| {
                matches!(
                    r.state,
                    RivalState::Come | RivalState::Face | RivalState::Lunge
                ) && (r.x - x).abs() < range
            })
            .map(|r| r.id)
    }

    /// Where a rival is, while it is still about.
    pub fn rival(&self, id: u32) -> Option<f32> {
        self.rivals
            .iter()
            .find(|r| r.id == id && r.state != RivalState::Dive)
            .map(|r| r.x)
    }

    /// The tiger has seen it off: the whole party runs, or dives.
    pub fn rout(&mut self, id: u32) {
        let kind = self.rivals.iter().find(|r| r.id == id).map(|r| r.kind);
        for r in &mut self.rivals {
            if Some(r.kind) == kind && r.state != RivalState::Flee && r.state != RivalState::Dive {
                r.state = if r.kind == RivalKind::Croc {
                    RivalState::Dive
                } else {
                    RivalState::Flee
                };
                r.steps = 0;
                r.dir = -r.dir;
                self.routed += 1;
            }
        }
    }

    pub fn dust(&mut self, x: f32, y: f32, n: usize, rng: &mut Rng) {
        for _ in 0..n {
            self.fleck(
                Fleck::Dust,
                (x + rng.rangef(-6.0, 6.0), y - rng.rangef(0.0, 3.0)),
                (rng.rangef(-0.4, 0.4), rng.rangef(-0.35, -0.05)),
                rng.range(18, 40) as u16,
                rng.u8(..),
            );
        }
    }

    /// A puff of marigold petals thrown up at `(x, y)`, to drift down.
    pub fn petals(&mut self, x: f32, y: f32, n: usize, rng: &mut Rng) {
        for _ in 0..n {
            self.fleck(
                Fleck::Petal,
                (x + rng.rangef(-5.0, 5.0), y + rng.rangef(-3.0, 3.0)),
                (rng.rangef(-0.2, 0.2), rng.rangef(-0.5, -0.15)),
                rng.range(90, 200) as u16,
                rng.u8(..),
            );
        }
    }

    pub fn splash(&mut self, x: f32, y: f32, rng: &mut Rng) {
        for _ in 0..2 {
            self.fleck(
                Fleck::Spray,
                (x + rng.rangef(-2.0, 2.0), y),
                (rng.rangef(-0.3, 0.3), rng.rangef(-0.9, -0.4)),
                rng.range(10, 18) as u16,
                rng.u8(..),
            );
        }
    }

    /// Leaves shaken loose from the canopy.
    pub fn leaves(&mut self, x: f32, n: usize, rng: &mut Rng) {
        for _ in 0..n {
            self.fleck(
                Fleck::Leaf,
                (x + rng.rangef(-90.0, 90.0), rng.rangef(4.0, 30.0)),
                (rng.rangef(-0.2, 0.2), rng.rangef(0.15, 0.35)),
                rng.range(150, 260) as u16,
                rng.u8(..),
            );
        }
    }

    fn fleck(
        &mut self,
        kind: Fleck,
        (x, y): (f32, f32),
        (vx, vy): (f32, f32),
        life: u16,
        shade: u8,
    ) {
        if self.specks.len() < 300 {
            self.specks.push(Speck {
                x,
                y,
                vx,
                vy,
                life,
                span: life,
                kind,
                shade,
            });
        }
    }
}

/// A bird takes off, away from `x`.
fn flush(b: &mut Bird, x: f32) {
    b.flying = true;
    b.dir = if b.x >= x { 1.0 } else { -1.0 };
    b.vx = b.dir * 1.3;
    b.vy = -1.1;
}

impl Jungle {
    /// The animals about when the scene starts.
    pub fn populate(&mut self, rng: &mut Rng) {
        self.life.rival_clock = rng.rangef(60.0, 160.0);
        let (vw, _) = self.view_size();
        let want = self.herds_wanted();
        for _ in 0..want {
            let x = self.tiger.x + rng.sign() * rng.rangef(vw * 0.45, vw * 1.3);
            self.add_herd(x, rng);
        }
        for _ in 0..self.flocks_wanted() {
            let x = self.tiger.x + rng.sign() * rng.rangef(vw * 0.3, vw * 1.4);
            self.add_flock(x, rng);
        }
    }

    fn herds_wanted(&self) -> usize {
        (2.0 * self.set.prey).round() as usize
    }

    fn flocks_wanted(&self) -> usize {
        (2.0 * self.set.prey).round() as usize
    }

    /// A herd of whatever lives here, around `x`, on dry land.
    fn add_herd(&mut self, x: f32, rng: &mut Rng) {
        if self.water(x).is_some() {
            return;
        }
        let species = if self.kind.snowy() {
            if rng.chance(0.62) {
                Species::RedDeer
            } else {
                Species::Boar
            }
        } else {
            match rng.u32(..10) {
                0..=4 => Species::Chital,
                5 | 6 => Species::Sambar,
                7 | 8 => Species::Boar,
                _ => Species::Peacock,
            }
        };
        let dir = rng.sign();
        let herd = self.life.id();
        for k in 0..species.herd(rng) {
            let bx = x + k as f32 * rng.rangef(14.0, 24.0) * rng.sign();
            if self.water(bx).is_some() {
                continue;
            }
            let id = self.life.id();
            self.life.beasts.push(Beast {
                id,
                species,
                x: bx,
                vx: 0.0,
                dir: if rng.chance(0.75) { dir } else { -dir },
                state: State::Graze,
                steps: 0,
                wait: rng.range(60, 240) as u32,
                phase: rng.f32(),
                stag: rng.chance(0.4),
                lift: 0.0,
                herd,
            });
        }
    }

    /// A flock of small birds pecking about on the ground near `x`.
    fn add_flock(&mut self, x: f32, rng: &mut Rng) {
        let kind = rng.usize(..art::BIRD_COLORS.len());
        for _ in 0..rng.range(3, 8) {
            self.life.birds.push(Bird {
                x: x + rng.rangef(-14.0, 14.0),
                y: FEET as f32 + rng.rangef(-1.0, 3.0),
                vx: 0.0,
                vy: 0.0,
                flying: false,
                kind,
                phase: rng.rangef(0.0, 6.3),
                dir: rng.sign(),
            });
        }
    }

    pub fn update_life(&mut self, rng: &mut Rng) {
        self.keep_population(rng);
        if !self.ufo.hushed() {
            self.update_beasts(rng);
            self.update_langurs(rng);
            self.update_birds(rng);
            self.update_rivals(rng);
        }
        for s in &mut self.life.specks {
            s.life = s.life.saturating_sub(1);
            s.x += s.vx;
            s.y += s.vy;
            match s.kind {
                Fleck::Dust => {
                    s.vx *= 0.95;
                    s.vy *= 0.96;
                }
                Fleck::Spray => s.vy += 0.09,
                Fleck::Leaf => s.vx = 0.35 * (s.life as f32 * 0.07 + s.shade as f32).sin(),
                Fleck::Petal => {
                    s.vx = 0.3 * (s.life as f32 * 0.09 + s.shade as f32).sin();
                    s.vy = (s.vy + 0.012).min(0.2);
                }
            }
        }
        self.life.specks.retain(|s| {
            s.life > 0 && (!matches!(s.kind, Fleck::Leaf | Fleck::Petal) || s.y < FEET as f32 + 4.0)
        });
    }

    /// Deals animals out around the camera, and lets the far ones go.
    fn keep_population(&mut self, rng: &mut Rng) {
        let (vw, _) = self.view_size();
        let cam = self.tiger_or_camera();
        let near = |x: f32| (x - cam).abs() < vw * 2.2;
        self.life.beasts.retain(|b| near(b.x));
        self.life.birds.retain(|b| near(b.x) && b.y > -10.0);
        self.life
            .rivals
            .retain(|r| near(r.x) && !(r.state == RivalState::Dive && r.steps > 40));
        if !self.tick.is_multiple_of(45) {
            return;
        }
        // A herd at a time comes in from beyond the edges of the view.
        let herds = {
            let mut ids: Vec<u32> = self
                .life
                .beasts
                .iter()
                .filter(|b| b.state != State::Flee)
                .map(|b| b.herd)
                .collect();
            ids.sort_unstable();
            ids.dedup();
            ids.len()
        };
        if herds < self.herds_wanted() {
            let x = cam + rng.sign() * rng.rangef(vw * 0.62, vw * 1.4);
            self.add_herd(x, rng);
        }
        let flocks = self.life.birds.iter().filter(|b| !b.flying).count() / 4;
        if flocks < self.flocks_wanted() {
            let x = cam + rng.sign() * rng.rangef(vw * 0.6, vw * 1.4);
            self.add_flock(x, rng);
        }
        // Langurs keep to the big trees, in the monsoon forest only.
        if !self.kind.snowy() && self.set.prey > 0.0 {
            let trunks = self.trunks(cam - vw * 1.2, cam + vw * 1.2);
            self.life
                .langurs
                .retain(|l| trunks.iter().any(|t| t.id == l.tree));
            self.life
                .langur_trees
                .retain(|id| trunks.iter().any(|t| t.id == *id));
            for t in trunks {
                if self.life.langur_trees.contains(&t.id) {
                    continue;
                }
                self.life.langur_trees.push(t.id);
                if unit(t.id as i32, 101, self.seed) > 0.55 * self.set.prey.min(1.8) {
                    continue;
                }
                for k in 0..rng.range(1, 4) {
                    let (h, len, side) = t.limbs[k as usize % 2];
                    let along = rng.rangef(3.0, len as f32 - 3.0);
                    let x0 = t.x + side as f32 * (t.w as f32 / 2.0 + along);
                    self.life.langurs.push(Langur {
                        x: x0,
                        y: (FEET - h - 10) as f32 - along / 3.0,
                        dir: -side as f32,
                        bark: 0,
                        phase: rng.rangef(0.0, 6.3),
                        tree: t.id,
                    });
                }
            }
        }
    }

    /// Where the animals are dealt out around: the tiger, or the camera
    /// while the tiger is away.
    fn tiger_or_camera(&self) -> f32 {
        if self.tiger.gone {
            self.view().x + self.view_size().0 / 2.0
        } else {
            self.tiger.x
        }
    }

    fn update_beasts(&mut self, rng: &mut Rng) {
        let t = &self.tiger;
        let (tx, mood, gone) = (t.x, t.mood, t.gone);
        let in_grass = self.grass(tx) > 8.0;
        // How close the tiger may come before a beast notices it.
        let sight = if gone {
            0.0
        } else {
            match mood {
                Mood::Charge | Mood::Pounce | Mood::Fend => 220.0,
                Mood::Stalk => {
                    if in_grass {
                        26.0
                    } else {
                        46.0
                    }
                }
                Mood::Nap | Mood::Feed | Mood::Carried | Mood::Look => 0.0,
                _ => 64.0,
            }
        };
        // Langurs barking nearby put everyone on edge.
        let barking = self
            .life
            .langurs
            .iter()
            .any(|l| l.bark > 0 && (l.x - tx).abs() < 180.0);
        let sight = if barking && sight > 0.0 {
            sight + 30.0
        } else {
            sight
        };
        let mut alarms: Vec<f32> = Vec::new();
        for i in 0..self.life.beasts.len() {
            let b = &mut self.life.beasts[i];
            b.steps += 1;
            let d = (b.x - tx).abs();
            if matches!(b.state, State::Graze | State::Walk) && d < sight {
                b.state = State::Alert;
                b.steps = 0;
                b.wait = rng.range(6, 18) as u32;
                b.dir = if tx > b.x { 1.0 } else { -1.0 };
                alarms.push(b.x);
            }
            let flight = b.species.flight();
            match b.state {
                State::Graze => {
                    b.vx *= 0.8;
                    if b.steps > b.wait {
                        b.state = State::Walk;
                        b.steps = 0;
                        b.wait = rng.range(30, 90) as u32;
                        if rng.chance(0.4) {
                            b.dir = -b.dir;
                        }
                    }
                }
                State::Walk => {
                    b.vx = b.dir * 0.32;
                    if b.steps > b.wait {
                        b.state = State::Graze;
                        b.steps = 0;
                        b.wait = rng.range(90, 300) as u32;
                    }
                }
                State::Alert => {
                    b.vx *= 0.7;
                    if b.steps > b.wait || d < 40.0 {
                        b.state = State::Flee;
                        b.steps = 0;
                        b.dir = if b.x >= tx { 1.0 } else { -1.0 };
                        self.life.bolted += 1;
                    }
                }
                State::Flee => {
                    b.vx += (b.dir * flight - b.vx) * 0.12;
                    if b.species == Species::Peacock {
                        b.lift = (b.lift + 0.5).min(26.0);
                    }
                }
            }
            b.x += b.vx;
            let stride = if b.state == State::Flee { 14.0 } else { 7.0 };
            b.phase = (b.phase + b.vx.abs() * 0.4 / stride).fract();
        }
        // Each alarm sets off the herd around it, with a cry.
        for ax in alarms {
            for b in &mut self.life.beasts {
                if matches!(b.state, State::Graze | State::Walk) && (b.x - ax).abs() < 70.0 {
                    b.state = State::Alert;
                    b.steps = 0;
                    b.wait = rng.range(4, 14) as u32;
                }
            }
            self.note(ax, FEET as f32 - 30.0, "!", Rgb(255, 226, 90), 40, 0.12);
        }
    }

    fn update_langurs(&mut self, rng: &mut Rng) {
        let (tx, gone, mood) = (self.tiger.x, self.tiger.gone, self.tiger.mood);
        let mut cries: Vec<(f32, f32)> = Vec::new();
        for l in &mut self.life.langurs {
            l.phase += 0.05;
            let near = !gone && (l.x - tx).abs() < 120.0 && !matches!(mood, Mood::Nap);
            if near && l.bark == 0 && rng.chance(0.05) {
                l.bark = rng.range(60, 140) as u32;
            }
            if l.bark > 0 {
                l.bark -= 1;
                l.dir = if tx > l.x { 1.0 } else { -1.0 };
                if l.bark % 24 == 0 {
                    cries.push((l.x, l.y - 12.0));
                }
            }
        }
        for (x, y) in cries {
            self.note(x, y, "!", Rgb(255, 196, 120), 26, 0.2);
        }
    }

    fn update_birds(&mut self, rng: &mut Rng) {
        let (tx, gone, mood) = (self.tiger.x, self.tiger.gone, self.tiger.mood);
        let range = match mood {
            Mood::Charge | Mood::Pounce | Mood::Fend => 90.0,
            Mood::Stalk => 26.0,
            _ => 44.0,
        };
        for b in &mut self.life.birds {
            b.phase += 0.3;
            if b.flying {
                b.x += b.vx;
                b.y += b.vy;
                b.vy = (b.vy + 0.02).min(-0.3);
            } else {
                // Hop about, pecking.
                if rng.chance(0.03) {
                    b.dir = -b.dir;
                }
                if rng.chance(0.06) {
                    b.x += b.dir;
                }
                if !gone && (b.x - tx).abs() < range && !matches!(mood, Mood::Nap | Mood::Feed) {
                    flush(b, tx);
                }
            }
        }
    }

    fn update_rivals(&mut self, rng: &mut Rng) {
        let (vw, _) = self.view_size();
        let t = &self.tiger;
        let (tx, tmood, tgone, tsink) = (t.x, t.mood, t.gone, t.sink);
        // Now and then a bear or a pack comes to try the tiger.
        if self.life.rival_clock <= 0.0
            && self.life.rivals.is_empty()
            && !tgone
            && tmood == Mood::Patrol
            && tsink == 0.0
        {
            self.life.rival_clock = rng.rangef(200.0, 420.0);
            let side = rng.sign();
            let start = tx + side * (vw * 0.62 + 40.0);
            if self.water(start).is_none() {
                if rng.chance(0.35) {
                    let id = self.life.id();
                    self.life.rivals.push(Rival {
                        id,
                        kind: RivalKind::Bear,
                        x: start,
                        dir: -side,
                        state: RivalState::Come,
                        steps: 0,
                        phase: 0.0,
                        speed: 0.7,
                        slot: 0.0,
                    });
                } else {
                    for k in 0..rng.range(3, 6) {
                        let id = self.life.id();
                        self.life.rivals.push(Rival {
                            id,
                            kind: RivalKind::Dog,
                            x: start + side * k as f32 * 16.0,
                            dir: -side,
                            state: RivalState::Come,
                            steps: 0,
                            phase: rng.f32(),
                            speed: 1.25,
                            slot: k as f32,
                        });
                    }
                }
            }
        }
        // A crocodile tries its luck with a tiger in or by its river.
        if !tgone && (tsink > 0.5 || tmood == Mood::Drink) {
            let jaws = self.tiger.jaws();
            if let Some(s) = self.water(jaws).or_else(|| self.water(tx))
                && s.croc
                && !self.life.croc_done.contains(&s.id)
                && !self.life.rivals.iter().any(|r| r.kind == RivalKind::Croc)
            {
                self.life.croc_done.push(s.id);
                if self.life.croc_done.len() > 32 {
                    self.life.croc_done.remove(0);
                }
                let side = if rng.chance(0.5) { 1.0 } else { -1.0 };
                let x = (jaws + side * 46.0).clamp(s.a + 20.0, s.b - 20.0);
                let id = self.life.id();
                self.life.rivals.push(Rival {
                    id,
                    kind: RivalKind::Croc,
                    x,
                    dir: if jaws > x { 1.0 } else { -1.0 },
                    state: RivalState::Lurk,
                    steps: 0,
                    phase: 0.0,
                    speed: 0.3,
                    slot: 0.0,
                });
            }
        }
        let jaws = self.tiger.jaws();
        let mut provoke: Option<u32> = None;
        for r in &mut self.life.rivals {
            r.steps += 1;
            r.phase = (r.phase + r.speed * 0.04).fract();
            match r.state {
                RivalState::Come => {
                    let want = tx - r.dir * (70.0 + r.slot * 14.0);
                    r.dir = if want > r.x { 1.0 } else { -1.0 };
                    r.x += r.dir * r.speed;
                    if (want - r.x).abs() < 3.0 || r.steps > 30 * 40 {
                        r.state = RivalState::Face;
                        r.steps = 0;
                        r.dir = if tx > r.x { 1.0 } else { -1.0 };
                    }
                }
                RivalState::Face => {
                    r.dir = if tx > r.x { 1.0 } else { -1.0 };
                    if r.steps > 20 {
                        provoke = Some(r.id);
                    }
                    // Nobody waits forever, not even for a tiger.
                    if r.steps > 30 * 30 {
                        r.state = RivalState::Flee;
                        r.dir = -r.dir;
                        r.steps = 0;
                    }
                }
                RivalState::Flee => {
                    let top = if r.kind == RivalKind::Dog { 3.2 } else { 2.6 };
                    r.speed += (top - r.speed) * 0.1;
                    r.x += r.dir * r.speed;
                }
                RivalState::Lurk => {
                    r.dir = if jaws > r.x { 1.0 } else { -1.0 };
                    r.x += r.dir * r.speed;
                    if (jaws - r.x).abs() < 22.0 {
                        r.state = RivalState::Lunge;
                        r.steps = 0;
                    }
                }
                RivalState::Lunge => {
                    if r.steps < 6 {
                        r.x += r.dir * 1.5;
                    }
                    provoke = Some(r.id);
                }
                RivalState::Dive => {
                    r.x -= r.dir * 0.5;
                }
            }
        }
        // The tiger never lets a challenge stand.
        if let Some(id) = provoke
            && !tgone
            && self.tiger.mood != Mood::Fend
            && !matches!(self.tiger.mood, Mood::Pounce | Mood::Look | Mood::Carried)
        {
            self.tiger.rival = Some(id);
            self.tiger.prey = None;
            self.tiger.set(Mood::Fend);
        }
    }

    /// Langurs, birds on the ground, the herds and the rivals.
    pub fn draw_life_back(&mut self) {
        let langurs = art::LANGUR_COLORS;
        let lut = self.lut(langurs);
        for i in 0..self.life.langurs.len() {
            let l = self.life.langurs[i];
            let frame = if l.bark > 0 && (l.bark / 6).is_multiple_of(2) {
                1
            } else {
                0
            };
            self.sprite(art::LANGUR[frame], l.x, l.y, l.dir, &lut);
        }
        for i in 0..self.life.beasts.len() {
            let b = self.life.beasts[i];
            self.draw_beast(&b);
        }
        for i in 0..self.life.rivals.len() {
            let r = self.life.rivals[i];
            if r.kind != RivalKind::Croc {
                self.draw_rival(&r);
            }
        }
        self.draw_birds(false);
    }

    /// The crocodile, which rises in front, and birds in the air.
    pub fn draw_life_front(&mut self) {
        for i in 0..self.life.rivals.len() {
            let r = self.life.rivals[i];
            if r.kind == RivalKind::Croc {
                self.draw_rival(&r);
            }
        }
        self.draw_birds(true);
    }

    fn draw_birds(&mut self, flying: bool) {
        for i in 0..self.life.birds.len() {
            let b = self.life.birds[i];
            if b.flying != flying {
                continue;
            }
            let lut = self.lut(art::BIRD_COLORS[b.kind]);
            let frame = if b.flying {
                1 + ((b.phase as u32) % 2) as usize
            } else {
                0
            };
            self.sprite(art::BIRD[frame], b.x, b.y, b.dir, &lut);
        }
    }

    fn draw_beast(&mut self, b: &Beast) {
        let mut lut = self.lut(b.species.palette());
        if !b.stag {
            lut[b'A' as usize] = None;
        }
        let y = FEET as f32 - b.lift;
        if b.species == Species::Peacock {
            let frame = if b.lift > 0.0 {
                2
            } else {
                ((b.phase * 2.0) as usize) % 2
            };
            self.sprite(art::PEACOCK[frame], b.x, y, b.dir, &lut);
            return;
        }
        let (build, body, mid): (&Build, Sprite, i32) = match b.species {
            Species::Boar => (&art::PIG, art::BOAR, 10),
            _ => {
                let grazing = b.state == State::Graze;
                (
                    &art::DEER,
                    if grazing {
                        art::CHITAL_GRAZE
                    } else {
                        art::CHITAL
                    },
                    12,
                )
            }
        };
        let legs = match b.state {
            State::Flee => Legs::Gait(
                if b.species == Species::Boar {
                    Gait::Gallop
                } else {
                    Gait::Bound
                },
                b.phase,
            ),
            State::Walk => Legs::Gait(Gait::Walk, b.phase),
            _ => Legs::Stand,
        };
        let fig = &mut self.fig;
        fig.clear();
        fig.far_legs(build, legs, 0.0);
        fig.near_legs(build, legs, 0.0);
        if let Some(tail) = &build.tail {
            fig.tail(
                tail,
                TailPose {
                    base: 2.4,
                    curl: 0.3,
                    sway: 0.5,
                    phase: b.phase * 12.0,
                },
            );
        }
        fig.stamp(body, 0, 0);
        let ground = build.ground as i32;
        // Bounding deer pitch up and down a little.
        let angle = if b.state == State::Flee && b.species != Species::Boar {
            0.12 * (b.phase * std::f32::consts::TAU).sin()
        } else {
            0.0
        };
        self.place(
            b.x,
            y,
            (mid, ground),
            b.dir,
            angle,
            (mid, ground / 2),
            &lut,
            None,
        );
    }

    fn draw_rival(&mut self, r: &Rival) {
        let snowy = self.kind.snowy() as usize;
        match r.kind {
            RivalKind::Croc => {
                let lut = self.lut(art::CROC_COLORS);
                let water = self.water_color();
                let (s, y) = match r.state {
                    RivalState::Lunge if r.steps < 40 => (art::CROC_LUNGE, FEET as f32 + 1.0),
                    RivalState::Dive => (art::CROC_LURK, FEET as f32 + 1.0 + r.steps as f32 * 0.15),
                    _ => (art::CROC_LURK, FEET as f32 + 1.0),
                };
                self.sprite(s, r.x, y, r.dir, &lut);
                // Its lower half stays under water.
                let w = s[0].len() as i32;
                for x in r.x as i32 - w / 2 - 1..=r.x as i32 + w / 2 + 1 {
                    for yy in FEET + 1..FEET + 4 {
                        self.blend(x, yy, water, 0.55);
                    }
                }
            }
            RivalKind::Bear => {
                let lut = self.lut(art::BEAR_COLORS[snowy]);
                let rearing = r.state == RivalState::Face && r.steps > 10;
                let legs = if rearing {
                    Legs::Rear(r.steps as f32 * 0.04)
                } else if r.state == RivalState::Face {
                    Legs::Stand
                } else {
                    Legs::Gait(
                        if r.state == RivalState::Flee {
                            Gait::Gallop
                        } else {
                            Gait::Walk
                        },
                        r.phase,
                    )
                };
                let b = &art::BEAR_BUILD;
                let fig = &mut self.fig;
                fig.clear();
                fig.far_legs(b, legs, 0.0);
                fig.near_legs(b, legs, 0.0);
                fig.stamp(art::BEAR, 0, 0);
                let ground = b.ground as i32;
                let (angle, pivot) = if rearing {
                    (REAR, (b.hind.hip.0 as i32, b.hind.hip.1 as i32))
                } else {
                    (0.0, (12, 6))
                };
                self.place(
                    r.x,
                    FEET as f32,
                    (12, ground),
                    r.dir,
                    angle,
                    pivot,
                    &lut,
                    None,
                );
                if rearing && r.steps % 30 == 11 {
                    let x = r.x + r.dir * 6.0;
                    self.note(x, FEET as f32 - 34.0, "!", Rgb(255, 160, 120), 24, 0.2);
                }
            }
            RivalKind::Dog => {
                let lut = self.lut(art::DOG_COLORS[snowy]);
                let legs = match r.state {
                    RivalState::Face => Legs::Stand,
                    RivalState::Flee => Legs::Gait(Gait::Gallop, r.phase),
                    _ => Legs::Gait(Gait::Trot, r.phase),
                };
                let b = &art::DOG;
                let fig = &mut self.fig;
                fig.clear();
                fig.far_legs(b, legs, 0.0);
                fig.near_legs(b, legs, 0.0);
                if let Some(tail) = &b.tail {
                    fig.tail(
                        tail,
                        TailPose {
                            base: if r.state == RivalState::Flee {
                                2.1
                            } else {
                                2.9
                            },
                            curl: 0.05,
                            sway: 0.4,
                            phase: r.phase * 20.0,
                        },
                    );
                }
                fig.stamp(art::DHOLE, 0, 0);
                let ground = b.ground as i32;
                self.place(
                    r.x,
                    FEET as f32,
                    (10, ground),
                    r.dir,
                    0.0,
                    (10, 6),
                    &lut,
                    None,
                );
            }
        }
    }

    pub fn draw_specks(&mut self) {
        let dust = self.lit(if self.kind.snowy() {
            Rgb(240, 244, 252)
        } else {
            Rgb(190, 164, 120)
        });
        let spray = self.water_color().lerp(Rgb::WHITE, 0.5);
        let mut leaves = [
            self.lit(Rgb(110, 150, 60)),
            self.lit(Rgb(196, 160, 60)),
            self.lit(Rgb(170, 96, 44)),
        ];
        // On Día de Muertos what falls from the trees is marigold, and at
        // Navidad the red and green and gold of the nochebuena.
        if let Some(d) = self.pal.fest {
            leaves = [d.marigold[1], d.marigold[2], d.marigold[0]].map(|c| self.lit(c));
        } else if let Some(d) = self.pal.navidad {
            leaves = [d.rojo, d.verde, d.dorado].map(|c| self.lit(c));
        }
        for i in 0..self.life.specks.len() {
            let s = self.life.specks[i];
            let (x, y) = (s.x.round() as i32, s.y.round() as i32);
            let k = s.life as f32 / s.span as f32;
            match s.kind {
                Fleck::Dust => self.blend(x, y, dust, 0.7 * k),
                Fleck::Spray => self.blend(x, y, spray, 0.8),
                Fleck::Leaf | Fleck::Petal => self.put(x, y, leaves[s.shade as usize % 3]),
            }
        }
    }
}
