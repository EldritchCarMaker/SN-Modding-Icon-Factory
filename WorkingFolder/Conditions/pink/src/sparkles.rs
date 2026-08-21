#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! rand = "0.10"
//! rand_distr = "0.6"
//! ```

use std::f64;
use rand::RngExt;

const WIDTH: u32 = 256;
const SPARKLES: usize = 128;
const CENTER: f64 = 64.0;
const ANGLE: (i32, i32) = (0, 45);
const SCALE: (f64, f64) = (0.15, 2.5);

fn main() {
    const HALFWIDTH: f64 = WIDTH as f64 / 2.0;
    const RADIUS: f64 = HALFWIDTH * f64::consts::SQRT_2;

    let mut rng = rand::make_rng::<rand::rngs::SmallRng>();
    let gamma = rand_distr::Gamma::new(1.0f64, 1.0).unwrap();

    struct Sparkle { x: i32, y: i32, a1: i32, a2: i32, s1: f64, s2: f64 }
    let mut sparkles = vec![];

    let mut done = 0;
    while done < SPARKLES {
        let theta = rng.random_range(0.0..f64::consts::TAU);
        let r = CENTER + (1.0 - rng.sample(gamma) / 4.0).max(0.0) * (RADIUS - CENTER);

        let x = (theta.cos() * r + HALFWIDTH).round();
        let y = (theta.sin() * r + HALFWIDTH).round();

        let s1 = (rng.sample(gamma) / 6.0 + SCALE.0).min(SCALE.1);

        let (dx, dy) = (x - x.clamp(0.0, WIDTH as f64), y - y.clamp(0.0, WIDTH as f64));
        if dx * dx + dy * dy < s1 * s1 {
            let a1 = rng.random_range(ANGLE.0..=ANGLE.1);
            let a2 = a1 + {
                if a1 < ANGLE.0 + 15 { 15 }
                else if a1 > ANGLE.1 - 15 { -15 }
                else if rng.random() { 15 }
                else { -15 }
            };

            let s2 = {
                let (grow, shrink) = (s1 * 1.1, s1 / 1.1);
                if shrink < SCALE.0 { grow }
                else if grow > SCALE.1 { shrink }
                else if rng.random() { grow }
                else { shrink }
            };

            let (x, y) = (x as i32, y as i32);
            sparkles.push(Sparkle { x, y, a1, a2, s1, s2 });
            done += 1;
        }
    }

    let print = |x, y, a, s: f64| {
        let tr = {
            if x == 0 && y == 0 { None }
            else { Some(format_args!("translate({x} {y})")) }
        };

        let rt = {
            if a == 0 { None }
            else { Some(format_args!("rotate({a})")) }
        };

        let s = (s * 100.0f64).round() / 100.0;
        let sc = {
            if s == 1.0 { None }
            else { Some(format_args!("scale({s})")) }
        };

        let attr = |tf| println!(r##"<use {tf}href="#sparkle"/>"##);
        match (tr, rt, sc) {
            (None, None, None) => attr(format_args!("")),
            (None, None, Some(sc)) => attr(format_args!(r#"transform="{sc}" "#)),
            (None, Some(rt), None) => attr(format_args!(r#"transform="{rt}" "#)),
            (None, Some(rt), Some(sc)) => attr(format_args!(r#"transform="{rt} {sc}" "#)),
            (Some(tr), None, None) => attr(format_args!(r#"transform="{tr}" "#)),
            (Some(tr), None, Some(sc)) => attr(format_args!(r#"transform="{tr} {sc}" "#)),
            (Some(tr), Some(rt), None) => attr(format_args!(r#"transform="{tr} {rt}" "#)),
            (Some(tr), Some(rt), Some(sc)) => attr(format_args!(r#"transform="{tr} {rt} {sc}" "#)),
        }
    };

    println!("<!-- twinkle frame one -->");
    for &Sparkle { x, y, a1, s1, .. } in &sparkles {
        print(x, y, a1, s1);
    }

    println!();
    println!("<!-- twinkle frame two -->");
    for &Sparkle { x, y, a2, s2, .. } in &sparkles {
        print(x, y, a2, s2);
    }
}
