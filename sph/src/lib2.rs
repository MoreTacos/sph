use cgmath::Rotation3;
use cgmath::Vector2;
use cgmath::InnerSpace;
use utils::Instance;
use rand::Rng;

const R: f32 = 0.005;
const MASS: f32 = 0.02;
const REST: f32 = 1000.0;
const H: f32 = 4*R;
const DT: f32 = 0.001;
const STIFF: f32 = 2000.0;
const VISC: f32 = 3000.0;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Particle {
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    rho: f32,
    p: f32,
    f: Vector2<f32>,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let pos = Vector2::new(x, y);
        let vel = Vector2::new(0.0, 0.0);
        let m = 1.0;
        let rho = 0.0;
        let p = 0.0;
        let f = Vector2::new(0.0, 0.0);
        Particle { pos, vel, m, rho, p, f }
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
                let mut rng = rand::thread_rng();
                let jitter = rng.gen_range(-0.02, 0.02);

                let p = Particle::new(x + jitter, y);
                particles.push(p);
            }
        }
       
        
        
        particles.push(Particle::new(0.375, 0.5));
        particles.push(Particle::new(0.525, 0.5));

        Self { particles }
    }

    pub fn integrate(&mut self, index: usize) {
        let mut p = self.particles[index];
        
        p.rho = 0.0;
        for &pi in &self.particles {
            let rij = pi.pos - p.pos;
            let r2 = rij.magnitude2();

            if r2 < DIAM2 {
                p.rho += p.m * POLY6 * (DIAM2 - r2) * (DIAM2 - r2) * (DIAM2 - r2);
            }
        }

        p.p = GAS_CONST * (p.rho - REST_DENS);

        let mut fpress = Vector2{ x: 0.0, y: 0.0 };
        let mut fvisc = Vector2{ x: 0.0, y: 0.0 };

        for &pi in &self.particles {
            if pi.pos == p.pos {
                continue;
            }

            let rij = pi.pos - p.pos;
            let r = rij.magnitude();

            if r < (R*2.0) {
                fpress += -rij.normalize() * p.m * (p.p + pi.p)/(2.0 * pi.rho) 
                    * SPIKY_GRAD * ((2.0*R) - r) * ((2.0*R) - r);
                fvisc += VISC * p.m * (p.vel - pi.vel)/pi.rho * VISC_LAP * ((2.0*R) - r);
            }
            let fgrav = G * p.rho;
            p.f = fpress + fvisc + fgrav;
        }

        p.vel += DT*p.f/p.rho;
        p.pos += DT*p.vel;

        
        if p.pos.x - R < 0.0 {
            p.vel.x *= BOUND_DAMPING;
            p.pos.x = R;
        }
        if p.pos.x + R > VIEW_WIDTH {
            p.vel.x *= BOUND_DAMPING;
            p.pos.x = VIEW_WIDTH - R;
        }
        if p.pos.y - R < 0.0 {
            p.vel.y *= BOUND_DAMPING;
            p.pos.y = R;
        }
        if p.pos.y + R > VIEW_HEIGHT {
            p.vel.y *= BOUND_DAMPING;
            p.pos.y = VIEW_HEIGHT - R;
        }

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
                    x: ((p.pos.x - 0.5) * 2.0),
                    y: ((p.pos.y - 0.5) * 2.0),
                    z: 0.0,
                },
                rotation: cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_z(),
                    cgmath::Deg(0.0),
                ),
                scale: 2.0 * cgmath::Vector2{x: R, y: R},
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
