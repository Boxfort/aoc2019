use super::ToPoints;

fn get_intersections(a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut intersections: Vec<(i32, i32)> = vec!();
    // Compare all pairs of points in each line to see if there's an intersection
    /*
    for i in 1..(a.len() - 1) {
        for j in 1..(b.len() - 1) {
            let xdiff = (a[i].0 - a[i+1].0, b[j].0 - b[j+1].0);
            let ydiff = (a[i].1 - a[i+1].1, b[j].1 - b[j+1].1);

            let det = |a: (i32,i32), b: (i32,i32)| { a.0 * b.1 - a.1 * b.0 };

            // Lines do not intersect
            let div = det(xdiff, ydiff);
            if div == 0 {
                break
            }

            let d: (i32, i32) = (det(a[i], a[i+1]), det(b[j], b[j+1]));
            let x = det(d, xdiff) / div;
            let y = det(d, ydiff) / div;

            println!("INTERSECTION");
            println!("a: {:?} - {:?}", a[i], a[i+1]);
            println!("b: {:?} - {:?}", b[j], b[j+1]);
            println!("p: {:?} - {:?}", x, y);


            intersections.push((x, y));
        }
    }
    */
    for i in 1..(a.len() - 1) {
        for j in 1..(b.len() - 1) {
            let its = Line(a[i], a[i+1]).intersect(Line(b[j], b[j+1]));
            if its.is_some() {
                intersections.push(its.unwrap());
            }
        }
    }

    intersections
}

struct Line((i32,i32),(i32,i32));

impl Line {
    pub fn intersect(self, other: Self) -> Option<(i32, i32)> {
        let a1 = (self.1).1 - (self.0).1;
        let b1 = (self.0).0 - (self.1).0;
        let c1 = a1 * (self.0).0 + b1 * (self.0).1;

        let a2 = (other.1).1 - (other.0).1;
        let b2 = (other.0).0 - (other.1).0;
        let c2 = a2 * (other.0).0 + b2 * (other.0).1;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0 {
            return None;
        }

        Some((
            ((b2 as i64 * c1 as i64 - b1 as i64 * c2 as i64) / delta as i64) as i32,
            ((a1 as i64 * c2 as i64 - a2 as i64 * c1 as i64) / delta as i64) as i32,
        ))
    }
}

fn manhattan_distance(point: &(i32, i32)) -> i32 {
    point.0.abs() + point.1.abs()
}

fn solve() -> i32 {
    // Get input points
    let input = super::get_input();

    // Check all lines to see if they intersect, if so get the point that they intersect at
    let intersections = get_intersections(&input[0].to_points(), &input[1].to_points());

    // Find the closest intersection to 0,0
    let min = manhattan_distance(&intersections[0]);
    for point in intersections.iter() {
        let md = manhattan_distance(point);
        if md < min {
            let min = md;
        }
    }
    min
}

mod tests {
    use super::solve;

    #[test]
    fn test_day3_part1() {
        let result = solve();
        assert_eq!(result, 4023471)
    }
}
