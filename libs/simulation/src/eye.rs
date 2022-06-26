use crate::*;
use std::f32::consts::*;

const FOV_RANGE: f32 = 0.25;

const FOV_ANGLE: f32 = PI + FRAC_PI_4;

const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye{
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}


impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {

        // checks preconditions 
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let relative_pos = food.position - position;

            let dist = relative_pos.norm();

            if dist>= self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(
                &na::Vector2::x(),
                &relative_pos,
            ).angle();

            // now we include the bird's rotation

            let angle = angle - rotation.angle();

            // and make it wrap around 

            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 ||
               angle > self.fov_angle / 2.0 
            {
                continue;
            }

            // now we make angle relative to bird.s FOV
            let angle = angle + self.fov_angle / 2.0;
            
            // we can then extract the index of the activated cell from the angle

            let cell_index = (angle/ self.fov_angle) * (self.cells as f32);

            // convert it to usize
            // and make sure it's within the bounds of the array

            let cell_index = (cell_index as usize).min(cells.len() - 1 );

            let energy = (self.fov_range - dist) / self.fov_range ;

            cells[cell_index] += energy;
        }
        cells 
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     struct TestCase {
//         foods: Vec<Food>,
//         fov_range: f32,
//         fov_angle: f32,
//         x: f32,
//         y: f32,
//         rot: f32,
//         expected_vision: &'static str,
//     }

//     impl TestCase {
//         fn run(self) {
//             todo!()
//         }
//     }

//     mod different_fov_ranges {
//         use super::*;
//         use test_case::test_case;

//         #[test_case(1.0)]
//         #[test_case(0.5)]
//         #[test_case(0.1)]
//         fn test(fov_range: f32) {
//             TestCase {
//                 foods: todo!(),
//                 fov_angle: todo!(),
//                 x: todo!(),
//                 y: todo!(),
//                 rot: todo!(),
//                 expected_vision: todo!(),
//                 fov_range,
//             }.run()
//         }
//     }
    
// }