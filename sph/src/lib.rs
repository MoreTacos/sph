use cgmath::Rotation3;
use utils::Instance;

pub struct Sph {
    pub particles: Vec<[f32; 2]>,
    pub timestep: fn(&Vec<[f32; 2]>) -> Vec<[f32; 2]>,
}

impl Sph {
    pub fn new(number_instances_per_row: i32) -> Self {
        let particles = vec![[0.5, 1.0], [0.5, 0.4], [0.0, 0.5]];

        fn timestep(particles: &Vec<[f32; 2]>) -> Vec<[f32; 2]> {
            particles
                .iter()
                .map(|p| [p[0] + 0.001, p[1] + 0.001])
                .collect()
        }
        Self {
            particles,
            timestep,
        }
    }
    pub fn instances(&self) -> Vec<Instance> {
        let instances = self.particles
            .iter()
            .map(|p| Instance {
                position: cgmath::Vector3 {
                    x: (p[0] - 0.5) * 2.0,
                    y: (p[1] - 0.5) * 2.0,
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
    pub fn timestep(&mut self) {
        self.particles = (self.timestep)(&self.particles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
