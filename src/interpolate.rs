#[derive(Debug)]
pub struct LerpIter {
    steps: usize,
    step_x: f32,
    step_y: f32,
    next: (f32, f32),
    num_steps: usize,
}

impl LerpIter {
    pub fn new(p1: (f32, f32), p2: (f32, f32), num_steps: usize) -> Self {
        let ((x1, y1), (x2, y2)) = match (p1, p2) {
            (pa, pb) if pa.0 < pb.0 => (pa, pb),
            (pa, pb) if pa.0 == pb.0 && pa.1 < pb.1 => (pa, pb),
            _ => (p2, p1),
        };

        Self {
            steps: 0,
            step_x: match num_steps {
                0 => 1.,
                1 => x2 - x1,
                _ => (x2 - x1) / (num_steps - 1) as f32,
            },
            step_y: match num_steps {
                0 => 1.,
                1 => y2 - y1,
                _ => (y2 - y1) / (num_steps - 1) as f32,
            },
            next: (x1, y1),
            num_steps,
        }
    }
}

impl Iterator for LerpIter {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == self.num_steps {
            return None;
        }

        let current = self.next;

        self.next.0 += self.step_x;
        self.next.1 += self.step_y;

        self.steps += 1;
        Some(current)
    }
}

#[cfg(test)]
mod tests {

    use std::{iter::zip, ops::RangeInclusive};

    use super::*;
    type Point = (f32, f32);

    fn point_range(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Vec<Point> {
        zip(x_range, y_range)
            .map(|(x, y)| (x as f32, y as f32))
            .collect()
    }

    #[test]
    fn basic() {
        assert!(true)
    }
    #[test]
    fn simple_lerp() {
        let zero_to_five: Vec<Point> = LerpIter::new((0., 0.), (5., 5.), 6).collect();
        let expected = point_range(0..=5, 0..=5);
        assert_eq!(expected, zero_to_five)
    }

    #[test]
    fn empty_lerp() {
        let empty: Vec<Point> = LerpIter::new((0., 0.), (5., 5.), 0).collect();
        assert_eq!(Vec::<Point>::new(), empty)
    }
    #[test]
    /// It's a bit ambigous if it should be the first or the last
    /// It seems a bit more predictable that the one value returned is
    /// the same as the first value for other numbers of steps
    ///
    /// Another option could be to return the average value
    fn one_lerp() {
        let one: Vec<Point> = LerpIter::new((0., 0.), (5., 5.), 1).collect();
        assert_eq!(vec![(0., 0.)], one)
    }

    #[test]
    fn swapped_input() {
        let zero_to_five: Vec<Point> = LerpIter::new((5., 5.), (0., 0.), 6).collect();
        let expected = point_range(0..=5, 0..=5);
        assert_eq!(expected, zero_to_five)
    }

    #[test]
    fn count_down() {
        let five_to_zero: Vec<Point> = LerpIter::new((0., 5.), (5., 0.), 6).collect();
        let expected: Vec<Point> = zip(0..=5, (0..=5).rev())
            .map(|(x, y)| (x as f32, y as f32))
            .collect();
        assert_eq!(expected, five_to_zero)
    }

    #[test]
    fn large_slope() {
        let zero_to_five: Vec<Point> = LerpIter::new((0., 0.), (1., 5.), 6).collect();
        assert_eq!(
            vec![
                (0.0, 0.0),
                (0.2, 1.0),
                (0.4, 2.0),
                (0.6, 3.0),
                (0.8, 4.0),
                (1.0, 5.0)
            ],
            zero_to_five
        )
    }
    #[test]
    fn horizontal_line() {
        let zero_to_five: Vec<Point> = LerpIter::new((0., 0.), (5., 0.), 6).collect();
        assert_eq!(
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (2.0, 0.0),
                (3.0, 0.0),
                (4.0, 0.0),
                (5.0, 0.0)
            ],
            zero_to_five
        )
    }
    #[test]
    /// When x1 == x2, it's ambigous whether to count down or up with y,
    /// we choose to count up
    fn vertical_line() {
        let vert: Vec<Point> = LerpIter::new((0., 0.), (0., 5.), 1).collect();
        assert_eq!((0.0, 0.0), vert[0])
    }
    #[test]
    /// swapping the points doesn't change the output
    fn vertical_line_2() {
        let vert: Vec<Point> = LerpIter::new((0., 5.), (0., 0.), 1).collect();
        assert_eq!((0.0, 0.0), vert[0])
    }
    #[test]
    fn vertical_line_3() {
        let vert: Vec<Point> = LerpIter::new((0., 5.), (0., 0.), 3).collect();
        assert_eq!(vec![(0.0, 0.0), (0.0, 2.5), (0.0, 5.0)], vert)
    }
}
