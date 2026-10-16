//! The technobabble engine: hand-written fragments, combined grammatically.
//!
//! Every table below is hand-written; the generator joins them through
//! sentence and tail templates, so a line almost never repeats: a noun phrase
//! alone has about 100 x 70 x 130 variants before a template is even chosen.
//! Lines are built into a caller-supplied `String` (born once, drawn many
//! times), never allocated per frame.
//!
//! The idea and the spirit of the "mundane" fragments follow Dustin
//! Kirkland's hollywood, <https://github.com/dustinkirkland/hollywood>,
//! Copyright 2014-2026 Dustin Kirkland, Apache License 2.0.

use fastrand::Rng;

/// Verbs as (base, gerund, past): "reroute", "rerouting", "rerouted".
pub const VERBS: &[(&str, &str, &str)] = &[
    ("reroute", "rerouting", "rerouted"),
    ("recompile", "recompiling", "recompiled"),
    ("spoof", "spoofing", "spoofed"),
    ("recalibrate", "recalibrating", "recalibrated"),
    ("reticulate", "reticulating", "reticulated"),
    ("defragment", "defragmenting", "defragmented"),
    ("decrypt", "decrypting", "decrypted"),
    ("encrypt", "encrypting", "encrypted"),
    ("bypass", "bypassing", "bypassed"),
    ("hijack", "hijacking", "hijacked"),
    ("triangulate", "triangulating", "triangulated"),
    ("synchronize", "synchronizing", "synchronized"),
    ("desynchronize", "desynchronizing", "desynchronized"),
    ("overclock", "overclocking", "overclocked"),
    ("reboot", "rebooting", "rebooted"),
    ("backtrace", "backtracing", "backtraced"),
    ("compress", "compressing", "compressed"),
    ("decompress", "decompressing", "decompressed"),
    ("flush", "flushing", "flushed"),
    ("purge", "purging", "purged"),
    ("quarantine", "quarantining", "quarantined"),
    ("tunnel into", "tunneling into", "tunneled into"),
    ("pivot through", "pivoting through", "pivoted through"),
    ("fingerprint", "fingerprinting", "fingerprinted"),
    ("handshake with", "handshaking with", "handshook with"),
    ("renegotiate", "renegotiating", "renegotiated"),
    ("inject", "injecting", "injected"),
    ("sanitize", "sanitizing", "sanitized"),
    ("obfuscate", "obfuscating", "obfuscated"),
    ("deobfuscate", "deobfuscating", "deobfuscated"),
    ("parse", "parsing", "parsed"),
    ("bootstrap", "bootstrapping", "bootstrapped"),
    ("calibrate", "calibrating", "calibrated"),
    ("transmogrify", "transmogrifying", "transmogrified"),
    ("hash", "hashing", "hashed"),
    ("salt", "salting", "salted"),
    ("fork", "forking", "forked"),
    ("daemonize", "daemonizing", "daemonized"),
    ("virtualize", "virtualizing", "virtualized"),
    ("containerize", "containerizing", "containerized"),
    ("partition", "partitioning", "partitioned"),
    ("remap", "remapping", "remapped"),
    ("realign", "realigning", "realigned"),
    ("reinitialize", "reinitializing", "reinitialized"),
    ("interrogate", "interrogating", "interrogated"),
    ("enumerate", "enumerating", "enumerated"),
    ("exfiltrate", "exfiltrating", "exfiltrated"),
    ("ping", "pinging", "pinged"),
    ("traceroute", "tracerouting", "traced"),
    ("amplify", "amplifying", "amplified"),
    ("attenuate", "attenuating", "attenuated"),
    ("modulate", "modulating", "modulated"),
    ("demodulate", "demodulating", "demodulated"),
    ("polarize", "polarizing", "polarized"),
    ("reverse", "reversing", "reversed"),
    ("invert", "inverting", "inverted"),
    ("entangle", "entangling", "entangled"),
    ("collapse", "collapsing", "collapsed"),
    ("vectorize", "vectorizing", "vectorized"),
    ("serialize", "serializing", "serialized"),
    ("deserialize", "deserializing", "deserialized"),
    ("rebase", "rebasing", "rebased"),
    ("patch", "patching", "patched"),
    ("jailbreak", "jailbreaking", "jailbroken"),
    ("root", "rooting", "rooted"),
    ("sandbox", "sandboxing", "sandboxed"),
    ("benchmark", "benchmarking", "benchmarked"),
    ("optimize", "optimizing", "optimized"),
    ("throttle", "throttling", "throttled"),
    ("load-balance", "load-balancing", "load-balanced"),
    ("garbage-collect", "garbage-collecting", "garbage-collected"),
    ("upload", "uploading", "uploaded"),
    ("download", "downloading", "downloaded"),
    ("mirror", "mirroring", "mirrored"),
    ("scan", "scanning", "scanned"),
    ("probe", "probing", "probed"),
    ("harden", "hardening", "hardened"),
    ("rewire", "rewiring", "rewired"),
    ("refactor", "refactoring", "refactored"),
    ("emulate", "emulating", "emulated"),
    ("decouple", "decoupling", "decoupled"),
    ("resolve", "resolving", "resolved"),
    ("terraform", "terraforming", "terraformed"),
    ("spin up", "spinning up", "spun up"),
    ("tokenize", "tokenizing", "tokenized"),
    ("checksum", "checksumming", "checksummed"),
    ("multiplex", "multiplexing", "multiplexed"),
    ("stabilize", "stabilizing", "stabilized"),
    (
        "recursively crawl",
        "recursively crawling",
        "recursively crawled",
    ),
    ("ionize", "ionizing", "ionized"),
];

/// Adjectives for noun phrases ("the polymorphic firewall").
pub const ADJ: &[&str] = &[
    "auxiliary",
    "polymorphic",
    "quantum",
    "heuristic",
    "recursive",
    "asynchronous",
    "redundant",
    "distributed",
    "encrypted",
    "fault-tolerant",
    "primary",
    "secondary",
    "tertiary",
    "biometric",
    "holographic",
    "hyperthreaded",
    "multiplexed",
    "virtual",
    "neural",
    "adaptive",
    "fractal",
    "stochastic",
    "deterministic",
    "orthogonal",
    "cryptographic",
    "binary",
    "ternary",
    "analog",
    "digital",
    "optical",
    "subspace",
    "interstellar",
    "orbital",
    "geostationary",
    "ionized",
    "plasma-cooled",
    "cryogenic",
    "thermal",
    "kinetic",
    "magnetic",
    "electromagnetic",
    "gravimetric",
    "isotropic",
    "tachyon",
    "positronic",
    "dilithium",
    "embedded",
    "legacy",
    "proprietary",
    "open-source",
    "classified",
    "top-secret",
    "unauthorized",
    "rogue",
    "sentient",
    "self-replicating",
    "autonomous",
    "anonymous",
    "obfuscated",
    "compressed",
    "overclocked",
    "unstable",
    "volatile",
    "persistent",
    "ephemeral",
    "immutable",
    "lock-free",
    "wait-free",
    "bit-sliced",
    "big-endian",
    "little-endian",
    "non-euclidean",
    "hyperbolic",
    "elliptic",
    "parametric",
    "algorithmic",
    "synthetic",
    "mainframe-grade",
    "military-grade",
    "enterprise-grade",
    "bare-metal",
    "serverless",
    "blockchain-backed",
    "AI-assisted",
    "cloud-native",
    "air-gapped",
    "reverse-engineered",
    "backdoored",
    "zero-day",
    "man-in-the-middle",
    "triple-redundant",
    "hot-swappable",
    "multi-threaded",
    "cross-platform",
    "context-aware",
    "turbo-charged",
    "nano-scale",
    "femtosecond",
    "superconducting",
    "photonic",
    "hypervisor-level",
    "kernel-mode",
    "ring-zero",
    "low-level",
    "high-bandwidth",
    "sub-atomic",
    "inter-dimensional",
    "metamorphic",
    "darknet",
    "dormant",
    "corrupted",
    "mission-critical",
    "real-time",
    "byzantine",
];

/// Noun adjuncts ("packet buffer", "root certificate"): put before a noun.
pub const MODS: &[&str] = &[
    "packet",
    "cache",
    "root",
    "kernel",
    "subnet",
    "proxy",
    "buffer",
    "socket",
    "thread",
    "pointer",
    "register",
    "pipeline",
    "firmware",
    "backplane",
    "mainframe",
    "gateway",
    "payload",
    "handshake",
    "token",
    "session",
    "cookie",
    "checksum",
    "hash",
    "cipher",
    "key",
    "certificate",
    "bitmap",
    "bandwidth",
    "latency",
    "plasma",
    "flux",
    "warp",
    "deflector",
    "shield",
    "phaser",
    "transporter",
    "tachyon",
    "quark",
    "photon",
    "neutrino",
    "dark-web",
    "botnet",
    "honeypot",
    "rootkit",
    "shellcode",
    "stack",
    "heap",
    "mutex",
    "semaphore",
    "interrupt",
    "syscall",
    "daemon",
    "cron",
    "loopback",
    "multicast",
    "broadcast",
    "uplink",
    "downlink",
    "satellite",
    "orbital",
    "telemetry",
    "sensor",
    "radar",
    "sonar",
    "ethernet",
    "fiber-optic",
    "superconductor",
    "capacitor",
    "matrix",
    "vector",
    "tensor",
    "gradient",
    "entropy",
    "hyperspace",
    "wormhole",
];

/// Nouns: subsystems, hardware and abstractions.
pub const NOUNS: &[&str] = &[
    "kernel",
    "firewall",
    "mainframe",
    "deflector array",
    "flux capacitor",
    "subnet mask",
    "hash table",
    "power grid",
    "backplane",
    "relay",
    "manifold",
    "matrix",
    "protocol",
    "algorithm",
    "heuristic",
    "compiler",
    "hypervisor",
    "scheduler",
    "dispatcher",
    "load balancer",
    "router",
    "switch",
    "modem",
    "gateway",
    "proxy server",
    "database",
    "data lake",
    "cache layer",
    "memory bus",
    "address space",
    "page table",
    "stack frame",
    "heap allocator",
    "garbage collector",
    "bootloader",
    "init system",
    "device driver",
    "file system",
    "inode table",
    "block device",
    "RAID array",
    "neural network",
    "inference engine",
    "decision tree",
    "genetic algorithm",
    "state machine",
    "message queue",
    "event loop",
    "thread pool",
    "mutex lock",
    "semaphore bank",
    "interrupt handler",
    "syscall table",
    "signal handler",
    "network stack",
    "TCP window",
    "DNS resolver",
    "certificate chain",
    "key schedule",
    "random number generator",
    "entropy pool",
    "cipher suite",
    "hash function",
    "digital signature",
    "blockchain",
    "ledger",
    "smart contract",
    "sandbox",
    "container",
    "virtual machine",
    "emulator",
    "debugger",
    "disassembler",
    "decompiler",
    "linker",
    "assembler",
    "preprocessor",
    "parser",
    "lexer",
    "bytecode interpreter",
    "JIT compiler",
    "runtime",
    "framework",
    "middleware",
    "API gateway",
    "microservice mesh",
    "webhook",
    "websocket",
    "uplink",
    "downlink",
    "antenna array",
    "satellite dish",
    "transponder",
    "oscillator",
    "waveform",
    "sensor grid",
    "targeting system",
    "navigation computer",
    "warp core",
    "plasma conduit",
    "dilithium chamber",
    "inertial dampener",
    "tractor beam",
    "cloaking device",
    "holodeck",
    "replicator",
    "transporter buffer",
    "main bus",
    "motherboard",
    "daughterboard",
    "co-processor",
    "GPU cluster",
    "quantum annealer",
    "qubit register",
    "superposition engine",
    "entanglement bridge",
    "wormhole stabilizer",
    "subspace transceiver",
    "neutrino detector",
    "particle accelerator",
    "fusion reactor",
    "cooling loop",
    "power supply",
    "uninterruptible power supply",
    "mesh network",
    "VPN tunnel",
    "honeypot",
    "botnet",
    "rootkit",
    "exploit chain",
    "payload",
    "backdoor",
    "firmware image",
    "microcode",
    "BIOS",
    "UEFI shell",
    "secure enclave",
    "trust anchor",
    "access control list",
    "intrusion detector",
    "packet filter",
    "sniffer",
    "log daemon",
    "mission clock",
    "core dump",
    "swap partition",
    "ramdisk",
    "terminal multiplexer",
];

/// Things that can be wrong with a noun: "the X is {state}".
pub const STATES: &[&str] = &[
    "overheating",
    "desynchronized",
    "oscillating",
    "leaking entropy",
    "hallucinating",
    "running out of tachyons",
    "stuck in a recursive loop",
    "fully compromised",
    "dangerously unstable",
    "vibrating at resonance",
    "emitting strange noises",
    "ignoring all commands",
    "out of phase",
    "on fire (metaphorically)",
    "self-aware",
    "refusing to cooperate",
    "operating at 110 percent",
    "negotiating with itself",
    "unreasonably quiet",
    "buffering",
    "deadlocked",
    "thrashing",
    "in a race condition",
    "dumping core",
    "waiting for quorum",
    "pending reboot",
    "going critical",
    "polarity-reversed",
    "exceeding design tolerances",
    "talking to the wrong subnet",
    "hot-swapped by mistake",
    "missing a semicolon",
    "mining something",
    "out of coffee",
    "phoning home",
    "learning too fast",
    "forgetting everything",
    "in sleep mode",
    "ready for the countdown",
    "locked on target",
];

/// Complete error messages for failures ("Failed to X: ...").
pub const ERRORS: &[&str] = &[
    "segmentation fault in the flux manifold",
    "buffer overflow at 0xDEADBEEF",
    "stack smashing detected",
    "kernel panic: not syncing",
    "illegal instruction",
    "bus error (core dumped)",
    "connection reset by peer",
    "no route to host",
    "certificate revoked by a higher power",
    "checksum mismatch",
    "quorum not reached",
    "heisenbug observed (now it works)",
    "entropy pool exhausted",
    "tachyon flux below minimum",
    "dilithium crystals at 3 percent",
    "rate limit exceeded",
    "access denied by the mainframe",
    "permission denied (try sudo harder)",
    "operation timed out in subspace",
    "temporal paradox detected",
    "out of memory: killing sentient process",
    "resource temporarily unavailable",
    "address already in use",
    "broken pipe",
    "protocol violation",
    "unexpected end of universe",
    "null pointer dereference",
    "use after free",
    "double free or corruption",
    "assertion failed: reality != illusion",
    "stale NFS handle",
    "device or resource busy",
    "input/output error",
    "cannot allocate memory",
    "too many open wormholes",
    "invalid magic number",
    "signature verification failed",
    "unknown cipher: ROT26",
    "firmware refused the update",
    "watchdog timer expired",
    "thermal limit reached",
    "uplink lost",
    "gimbal lock",
    "floating point exception",
    "numerical instability",
    "interrupt storm",
    "split brain detected",
    "clock skew exceeds 3 parsecs",
    "hash collision (impossible)",
    "backdoor already installed by somebody else",
];

/// Units for fake measurements.
pub const UNITS: &[&str] = &[
    "THz",
    "GHz",
    "MHz",
    "kHz",
    "TB/s",
    "GB/s",
    "Mbit/s",
    "petaflops",
    "gigawatts",
    "megajoules",
    "teraflops",
    "parsecs",
    "light-years",
    "milliseconds",
    "nanoseconds",
    "picoseconds",
    "femtoseconds",
    "qubits",
    "megaparsecs",
    "kelvin",
    "sieverts",
    "teslas",
    "decibels",
    "gigabytes",
    "terabytes",
    "petabytes",
    "kilobaud",
    "rpm",
    "cycles",
    "teraflops/watt",
];

/// Time units for "for N ...".
pub const TIMES: &[&str] = &[
    "milliseconds",
    "seconds",
    "microseconds",
    "clock cycles",
    "heartbeats",
    "nanoseconds",
    "jiffies",
    "epochs",
];

/// Daemons and process names for log lines.
pub const DAEMONS: &[&str] = &[
    "kerneld",
    "flux-syncd",
    "warpd",
    "sshd",
    "crond",
    "firewalld",
    "deflectord",
    "quantum-dnsd",
    "tachyon-relay",
    "dhcpd",
    "neurald",
    "hexd",
    "cipherd",
    "packetd",
    "rootkitd",
    "matrixd",
    "mainframed",
    "subspaced",
    "holodeckd",
    "replicatord",
    "systemd",
    "udevd",
    "NetworkManager",
    "dbus-daemon",
    "journald",
    "auditd",
    "polkitd",
    "chronyd",
    "named",
    "nginx",
    "postfix",
    "redis-server",
    "etcd",
    "consul",
    "vaultd",
    "docker",
    "containerd",
    "kubelet",
    "iptables",
    "snort",
    "fail2ban",
    "tripwire",
    "zerodayd",
    "botnetd",
    "honeypotd",
    "proxyd",
    "gatewayd",
    "uplinkd",
    "telemetryd",
    "plasmad",
    "dilithiumd",
    "navcomputerd",
    "targetingd",
    "cloakd",
    "entropyd",
    "mined",
    "backdoord",
    "cache-warmerd",
    "overclockd",
];

/// Plain syslog lines for `babble = 0`, so the scene still looks busy.
pub const MUNDANE: &[&str] = &[
    "session opened for user root",
    "connection closed by remote host",
    "accepted publickey for admin",
    "link is up, 1000 Mbit/s, full duplex",
    "started periodic cleanup",
    "cache hit ratio 97 percent",
    "reloading configuration",
    "disk usage within limits",
    "time synchronized with upstream",
    "worker process exited normally",
    "listening on all interfaces",
    "heartbeat ok",
    "lease renewed",
    "rotating logs",
    "queue drained",
    "health check passed",
];

/// Determiners for noun phrases.
const DETS: &[&str] = &[
    "the",
    "the",
    "the",
    "the",
    "the",
    "our",
    "your",
    "their",
    "that",
    "this",
    "each",
    "every",
    "the entire",
    "the remaining",
    "the primary",
    "the unauthorized",
];

/// Picks one element.
#[inline]
pub fn pick<'a>(rng: &mut Rng, list: &'a [&'a str]) -> &'a str {
    list[rng.usize(..list.len())]
}

#[inline]
fn verb(rng: &mut Rng) -> (&'static str, &'static str, &'static str) {
    VERBS[rng.usize(..VERBS.len())]
}

/// Upper-cases the first letter of `s[from..]` in place.
fn capitalize_from(s: &mut String, from: usize) {
    if let Some(c) = s[from..].chars().next()
        && c.is_ascii_lowercase()
    {
        s.replace_range(from..from + 1, &c.to_ascii_uppercase().to_string());
    }
}

/// "4.2 THz", "17 petaflops": a believable measurement.
pub fn measure(rng: &mut Rng, out: &mut String) {
    use std::fmt::Write;
    let _ = match rng.u8(..4) {
        0 => write!(out, "{}", rng.u32(2..999)),
        1 => write!(out, "{}.{}", rng.u32(1..99), rng.u32(0..9)),
        2 => write!(out, "{}.{:02}", rng.u32(0..9), rng.u32(0..99)),
        _ => write!(out, "{}", rng.u32(10..9999)),
    };
    out.push(' ');
    out.push_str(pick(rng, UNITS));
}

/// "the polymorphic packet firewall"; a high `level` adds a second adjective.
pub fn noun_phrase(rng: &mut Rng, level: u8, out: &mut String) {
    out.push_str(pick(rng, DETS));
    out.push(' ');
    out.push_str(pick(rng, ADJ));
    out.push(' ');
    if level >= 7 && rng.u8(..10) < 4 {
        out.push_str(pick(rng, ADJ));
        out.push(' ');
    }
    if rng.bool() {
        out.push_str(pick(rng, MODS));
        out.push(' ');
    }
    out.push_str(pick(rng, NOUNS));
}

/// A trailing clause: "through the X", "with Y flags", "at 4 THz"...
fn tail(rng: &mut Rng, level: u8, out: &mut String) {
    use std::fmt::Write;
    out.push(' ');
    match rng.u8(..14) {
        0 => {
            let _ = write!(out, "with {} {} flags", pick(rng, ADJ), pick(rng, MODS));
        }
        1 => {
            out.push_str("through ");
            noun_phrase(rng, level, out);
        }
        2 => {
            out.push_str("via ");
            noun_phrase(rng, level, out);
        }
        3 => {
            let _ = write!(
                out,
                "using {} {} {}",
                pick(rng, ADJ),
                pick(rng, MODS),
                pick(rng, NOUNS)
            );
        }
        4 => {
            out.push_str("across ");
            noun_phrase(rng, level, out);
        }
        5 => {
            out.push_str("inside ");
            noun_phrase(rng, level, out);
        }
        6 => {
            let _ = write!(out, "by {} ", verb(rng).1);
            noun_phrase(rng, level, out);
        }
        7 => {
            out.push_str("at ");
            measure(rng, out);
        }
        8 => {
            let _ = write!(out, "for {} {}", rng.u32(2..9999), pick(rng, TIMES));
        }
        9 => {
            out.push_str("against ");
            noun_phrase(rng, level, out);
        }
        10 => {
            let _ = write!(out, "under {} conditions", pick(rng, ADJ));
        }
        11 => {
            let _ = write!(out, "in {} mode", pick(rng, ADJ));
        }
        12 => {
            let _ = write!(out, "(code 0x{:04X})", rng.u16(..));
        }
        _ => {
            let _ = write!(out, "on port {}", rng.u16(1..65535));
        }
    }
}

/// "Rerouting the auxiliary power grid through the deflector array" (no final period).
pub fn gerund_phrase(rng: &mut Rng, level: u8, out: &mut String) {
    let from = out.len();
    out.push_str(verb(rng).1);
    out.push(' ');
    noun_phrase(rng, level, out);
    for _ in 0..rng.u8(..=(level / 3).min(3)) {
        tail(rng, level, out);
    }
    capitalize_from(out, from);
}

/// A whole technobabble sentence, in one of a dozen shapes.
pub fn sentence(rng: &mut Rng, level: u8, out: &mut String) {
    use std::fmt::Write;
    let level = level.min(10);
    let from = out.len();
    if level == 0 {
        out.push_str(pick(rng, MUNDANE));
        capitalize_from(out, from);
        return;
    }
    match rng.u8(..12) {
        0..=2 => gerund_phrase(rng, level, out),
        3 => {
            let _ = write!(out, "{} ", verb(rng).1);
            noun_phrase(rng, level, out);
            let _ = write!(out, " and {} ", verb(rng).1);
            noun_phrase(rng, level, out);
        }
        4 => {
            let _ = write!(out, "failed to {} ", verb(rng).0);
            noun_phrase(rng, level, out);
            let _ = write!(out, ": {}", pick(rng, ERRORS));
        }
        5 => {
            noun_phrase(rng, level, out);
            let _ = write!(out, " was {} successfully (", verb(rng).2);
            measure(rng, out);
            out.push(')');
        }
        6 => {
            out.push_str("warning: ");
            noun_phrase(rng, level, out);
            let _ = write!(out, " is {}", pick(rng, STATES));
        }
        7 => {
            out.push_str("engaging ");
            noun_phrase(rng, level, out);
            for _ in 0..rng.u8(..=(level / 3).min(3)) {
                tail(rng, level, out);
            }
        }
        8 => {
            noun_phrase(rng, level, out);
            let _ = write!(out, " is {}; {} ", pick(rng, STATES), verb(rng).1);
            noun_phrase(rng, level, out);
        }
        9 => {
            let _ = write!(out, "please {} ", verb(rng).0);
            noun_phrase(rng, level, out);
            out.push_str(" before ");
            noun_phrase(rng, level, out);
            let _ = write!(out, " is {}", pick(rng, STATES));
        }
        10 => {
            let _ = write!(out, "{} ", verb(rng).1);
            noun_phrase(rng, level, out);
            out.push_str(" at ");
            measure(rng, out);
        }
        _ => {
            let _ = write!(out, "unable to {} ", verb(rng).0);
            noun_phrase(rng, level, out);
            let _ = write!(out, " (retry in {} {})", rng.u32(2..900), pick(rng, TIMES));
        }
    }
    capitalize_from(out, from);
}

/// A short lower-case name for a pane title or a task: "quantum flux manifold".
pub fn title(rng: &mut Rng, out: &mut String) {
    out.push_str(pick(rng, ADJ));
    out.push(' ');
    if rng.bool() {
        out.push_str(pick(rng, MODS));
        out.push(' ');
    }
    out.push_str(pick(rng, NOUNS));
}

/// An UPPER-CASE progress label: "BYPASSING THE POLYMORPHIC FIREWALL".
pub fn progress_label(rng: &mut Rng, level: u8, out: &mut String) {
    let from = out.len();
    out.push_str(verb(rng).1);
    out.push(' ');
    noun_phrase(rng, level.min(4), out);
    out[from..].make_ascii_uppercase();
}

/// A dotted IPv4 address.
pub fn ip(rng: &mut Rng, out: &mut String) {
    use std::fmt::Write;
    let _ = write!(
        out,
        "{}.{}.{}.{}",
        rng.u8(1..=254),
        rng.u8(..=255),
        rng.u8(..=255),
        rng.u8(1..=254)
    );
}

/// Number of hand-written fragments (for the tests).
#[cfg(test)]
pub fn fragment_count() -> usize {
    VERBS.len() * 3
        + ADJ.len()
        + MODS.len()
        + NOUNS.len()
        + STATES.len()
        + ERRORS.len()
        + UNITS.len()
        + TIMES.len()
        + DAEMONS.len()
        + MUNDANE.len()
        + DETS.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn distinct(level: u8) -> usize {
        let mut rng = Rng::with_seed(5);
        let mut set = HashSet::new();
        let mut s = String::new();
        for _ in 0..10_000 {
            s.clear();
            sentence(&mut rng, level, &mut s);
            set.insert(s.clone());
        }
        set.len()
    }

    #[test]
    fn enormous_variety() {
        for level in [3, 7, 10] {
            assert!(distinct(level) >= 9_000, "level {level}");
        }
        assert!(fragment_count() >= 800);
    }

    #[test]
    fn lines_are_clean() {
        let mut rng = Rng::with_seed(1);
        let mut s = String::new();
        for _ in 0..500 {
            s.clear();
            sentence(&mut rng, 10, &mut s);
            assert!(!s.contains("  ") && !s.is_empty());
        }
    }
}
