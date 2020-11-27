use cgmath::Rotation3;
use utils::Instance;

pub struct Sph {
    number_of_particles: i32
}

impl Sph {
    pub fn new(number_instances_per_row: i32) -> (Vec<Instance>, fn(&Vec<Instance>) -> Vec<Instance>) {
        let instances: Vec<Instance> = (0..number_instances_per_row)
        .flat_map(|y| {
            (0..number_instances_per_row).map(move |x| {
                let x = (x as f32 / 10.0);
                let y = (y as f32 / 10.0);
                let position = cgmath::Vector3 { x, y, z: 0.0 };

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_z(),
                    cgmath::Deg(0.0),
                );

                Instance { position, rotation }
            })
        })
        .collect::<Vec<_>>();
        
        fn timestep(instances: &Vec<Instance>) -> Vec<Instance> {
            instances.iter().map(|instance| Instance {
                position: instance.position + cgmath::Vector3{ x:0.01, y: 0.0, z: 0.0},
                rotation: instance.rotation,
            }).collect()
        }

        (instances, timestep)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
