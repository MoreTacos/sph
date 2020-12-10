use cgmath::Rotation3;
use cgmath::Vector2;
use utils::Instance;

const TIMESTEP: f32 = 0.004;
const SCALE_CONSTANT: f32 = 1.8;
const GRAVITY: f32 = -9.8;
const BOUND_DAMPING: f32 = -0.5;
const VIEW_WIDTH: f32 = 1.0;
const VIEW_HEIGHT: f32 = 1.0;

#[derive(Copy, Clone, PartialEq)]
pub struct Particle {
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    m: f32,
    scale: Vector2<f32>,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let pos = Vector2::new(x, y);
        let vel = Vector2::new(0.0, 0.0);
        let m = 1.0;
        let scale = Vector2::new(0.01, 0.01);
        Particle { pos, vel, m, scale }
    }
    fn abs_dist(&self, p: &Particle) -> f32 {
        (self.pos.x * self.pos.x + self.pos.y * self.pos.y).sqrt() -
        (p.pos.x * p.pos.x + p.pos.y * p.pos.y).sqrt()
    }
}

pub struct Sph {
    pub particles: Vec<Particle>,
}

impl Sph {
    pub fn new(number_instances_per_row: i32) -> Self {
        let mut particles = vec![];

        
        let center_x = VIEW_WIDTH / 2.0;
        let quarter_x = VIEW_WIDTH / 4.0;
        let dist_x = center_x / number_instances_per_row as f32;

        let center_y = VIEW_HEIGHT / 2.0;
        let quarter_y = VIEW_HEIGHT / 4.0;
        let dist_y = center_y / number_instances_per_row as f32;

        for i in 0..number_instances_per_row {
            let x = quarter_x + i as f32 * dist_x;
            for j in 0..number_instances_per_row {
                let y = quarter_y + j as f32 * dist_y;

                let p = Particle::new(x, y);
                particles.push(p);
            }
        }
        

        particles.push(Particle::new(0.5, 0.5));
        particles.push(Particle::new(0.49, 0.5));

        Self { particles }
    }

    pub fn integrate(&mut self, index: usize) {
        let mut p = self.particles[index];

        p.pos.x = p.pos.x + p.vel.x * TIMESTEP;
        p.pos.y = p.pos.y + p.vel.y * TIMESTEP;

        if p.pos.x - p.scale.x < 0.0 {
            p.vel.x *= BOUND_DAMPING;
            p.pos.x = p.scale.x;
        }
        if p.pos.x + 0.01 > VIEW_WIDTH {
            p.vel.x *= BOUND_DAMPING;
            p.pos.x = VIEW_WIDTH - p.scale.x;
        }
        if p.pos.y - 0.01 < 0.0 {
            p.vel.y *= BOUND_DAMPING;
            p.pos.y = p.scale.y;
        }
        if p.pos.y + 0.01 > VIEW_HEIGHT {
            p.vel.y *= BOUND_DAMPING;
            p.pos.y = VIEW_HEIGHT - p.scale.y;
        }

        let gravity = Vector2 { x: 0.0, y: GRAVITY };
        let mut pressure = Vector2 { x: 0.0, y: 0.0 };
        let mut viscosity = Vector2 { x: 0.0, y: 0.0 };
        let mut n = vec![];
        
        for &pi in &self.particles {
            if pi == p {
                continue;
            }

            if (p.abs_dist(&pi)).abs() < p.scale.x {
                n.push(pi);
            }
        }

        for pi in n {
            pressure.x += (p.pos.x - pi.pos.x) * 1.0;
            pressure.y += (p.pos.y - pi.pos.y) * 1.0;

            viscosity.x += (p.vel.x - pi.vel.x) * -0.5;
            viscosity.y += (p.vel.y - pi.vel.x) * -0.5;
        }

        let forces = gravity + pressure + viscosity;

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
                scale: p.scale * SCALE_CONSTANT,
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
