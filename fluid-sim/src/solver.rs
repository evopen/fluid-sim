use std::{iter::successors, ops::Deref};

use once_cell::sync::Lazy;
use rayon::prelude::*;

use glam::Vec2;
use glam::Vec3A as Vec3;

use bytemuck::{Pod, Zeroable};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const VIEW_WIDTH: f32 = WINDOW_WIDTH as f32 * 1.5;
const VIEW_HEIGHT: f32 = WINDOW_HEIGHT as f32 * 1.5;
const REST_DENS: f32 = 1000.0;
const H: f32 = 16.0;
const HSQ: f32 = H * H;
const MASS: f32 = 65.0;
const GAS_CONST: f32 = 2000.0;
const POLY6: Lazy<f32> = Lazy::new(|| 315.0 / (65.0 * std::f32::consts::PI * H.powi(9)));
const SPIKY_GRAD: Lazy<f32> = Lazy::new(|| -45.0 / (std::f32::consts::PI * H.powi(6)));
const VISC_LAP: Lazy<f32> = Lazy::new(|| 45.0 / (std::f32::consts::PI * H.powi(6)));
const VISC: f32 = 250.0;
const G: Vec2 = glam::const_vec2!([0.0, 12000.0 * -9.8]);
const DT: f32 = 0.0008;
const BOUND_DAMPING: f32 = -0.5;

#[repr(C)]
#[derive(Debug, Default, Pod, Zeroable, Clone, Copy)]
pub struct Particle {
    pos: Vec2,
    v: Vec2,
    f: Vec2,
    rho: f32,
    p: f32,
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }
}

pub struct Solver {
    pub particles: Vec<Particle>,
}

impl Solver {
    pub fn new(count: u32) -> Self {
        let mut particles = Vec::with_capacity(count as usize);

        for y in std::iter::successors(Some(H), |y| Some(y + H - 0.01))
            .take_while(|y| y < &(VIEW_HEIGHT - 2.0 * H))
        {
            for x in std::iter::successors(Some(VIEW_WIDTH / 4.0), |x| Some(x + H - 0.01))
                .take_while(|x| x < &(VIEW_WIDTH / 2.0))
            {
                if particles.len() >= count as usize {
                    break;
                }
                particles.push(Particle::new(Vec2::new(x, y)));
            }
        }
        println!("initialize dam break with {} particles", particles.len());

        Self { particles }
    }

    fn compute_density_pressure(&mut self) {
        let particles_cache = self.particles.clone();
        self.particles.par_iter_mut().for_each(|pi| {
            pi.rho = 0.0;
            for pj in &particles_cache {
                let r2 = pi.pos.distance_squared(pj.pos);
                if r2 < HSQ {
                    pi.rho += MASS * *POLY6 * (HSQ - r2).powi(3);
                }
            }
            pi.p = GAS_CONST * (pi.rho - REST_DENS);
        });
    }

    fn compute_forces(&mut self) {
        let particles_cache = self.particles.clone();
        self.particles
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pi)| {
                let mut fpress = Vec2::new(0.0, 0.0);
                let mut fvisc = Vec2::new(0.0, 0.0);

                for (j, pj) in particles_cache.iter().enumerate() {
                    if j.eq(&i) {
                        continue;
                    }
                    let rij: Vec2 = pj.pos - pi.pos;

                    let r = pi.pos.distance(pj.pos);

                    if r < H {
                        fpress += -rij.normalize() * MASS * (pi.p + pj.p) / (2.0 * pj.rho)
                            * *SPIKY_GRAD
                            * (H - r).powi(2);

                        fvisc += VISC * MASS * (pj.v - pi.v) / pj.rho * *VISC_LAP * (H - r);
                    }
                }

                let fgrav = G * pi.rho;

                pi.f = fgrav + fpress + fvisc;
            });
    }

    pub fn integrate(&mut self) {
        self.particles.par_iter_mut().for_each(|p| {
            p.v += DT * p.f / p.rho;
            p.pos += DT * p.v;

            if p.pos.x - H < 0.0 {
                p.v.x *= BOUND_DAMPING;
                p.pos.x = H;
            }
            if p.pos.x + H > VIEW_WIDTH {
                p.v.x *= BOUND_DAMPING;
                p.pos.x = VIEW_WIDTH - H;
            }
            if p.pos.y - H < 0.0 {
                p.v.y *= BOUND_DAMPING;
                p.pos.y = H;
            }
            if p.pos.y + H > VIEW_HEIGHT {
                p.v.y *= BOUND_DAMPING;
                p.pos.y = VIEW_HEIGHT - H;
            }
        });
    }

    pub fn update(&mut self) {
        self.compute_density_pressure();
        self.compute_forces();
        self.integrate();
    }
}
