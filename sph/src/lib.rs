use cgmath::Rotation3;
use cgmath::Vector2;
use utils::Instance;

const TIMESTEP: f32 = 0.05;
const GRAVITY: f32 = -9.8;
const BOUND_DAMPING: f32 = -0.5;
const R: f32 = 0.01;
const VIEW_WIDTH: f32 = 1.0;
const VIEW_HEIGHT: f32 = 1.0;

#[derive(Copy, Clone, PartialEq)]
pub struct Particle {
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    m: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let pos = Vector2::new(x, y);
        let vel = Vector2::new(0.0, 0.0);
        let m = 1.0;
        Particle { pos, vel, m }
    }
    fn abs(&self) -> f32 {
        (self.pos.x * self.pos.x + self.pos.y * self.pos.y).sqrt()
    }
}

pub struct Sph {
    pub particles: Vec<Particle>,
}

impl Sph {
    pub fn new(_number_instances_per_row: i32) -> Self {
        let particles = vec![Particle::new(0.5, 1.0), Particle::new(0.49, 0.99)];

        Self { particles }
    }
    pub fn integrate(&mut self, index: usize) {
        let mut p = self.particles[index];

        p.pos.x = p.pos.x + p.vel.x * TIMESTEP;
        p.pos.y = p.pos.y + p.vel.y * TIMESTEP;

        if p.pos.x - R < 0.0 {
            p.vel.x *= BOUND_DAMPING;
            p.pos.x = R;
        }
        if p.pos.x + 0.01 > VIEW_WIDTH {
            p.vel.x *= BOUND_DAMPING;
            p.pos.x = VIEW_WIDTH - R;
        }
        if p.pos.y - 0.01 < 0.0 {
            p.vel.y *= BOUND_DAMPING;
            p.pos.y = R;
        }
        if p.pos.y + 0.01 > VIEW_HEIGHT {
            p.vel.y *= BOUND_DAMPING;
            p.pos.y = VIEW_HEIGHT - R;
        }

        let gravity = Vector2 { x: 0.0, y: GRAVITY };
        let mut pressure = Vector2 { x: 0.0, y: 0.0 };

        for &pi in &self.particles {
            if pi == p {
                continue;
            }

            if (p.abs() - pi.abs()).abs() < R {
                pressure.x += (p.pos.x - pi.pos.x) * p.m * 10.0;
                pressure.y += (p.pos.y - pi.pos.y) * p.m * 10.0;
            }
        }
        println!("{:?}", pressure);

        let forces = gravity + pressure;

        p.vel.x = p.vel.x + (forces.x / p.m) * TIMESTEP;
        p.vel.y = p.vel.y + (forces.y / p.m) * TIMESTEP;

        self.particles[index] = p;
    }
    pub fn timestep(&mut self) {
        for p in 0..self.particles.len() {
            self.integrate(p);
        }
    }
    pub fn instances(&self) -> Vec<Instance> {
        let instances = self
            .particles
            .iter()
            .map(|p| Instance {
                position: cgmath::Vector3 {
                    x: (p.pos.x - 0.5) * 2.0,
                    y: (p.pos.y - 0.5) * 2.0,
                    z: 0.0,
                },
                rotation: cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_z(),
                    cgmath::Deg(0.0),
                ),
            })
            .collect::<Vec<_>>();
        instances
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
