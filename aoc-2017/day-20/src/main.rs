use std::collections::{HashMap, HashSet};
use std::ops::Sub;

#[derive(Clone, Copy)]
struct Vector {
    vals: [i32; 3], // [x, y, z]
}

impl From<&str> for Vector {
    /// Creates a Vector from a string of the form "<x,y,z>".
    fn from(value: &str) -> Self {
        let vals = value
            .trim_matches(['<', '>'])
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { vals }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let vals = (0..3)
            .map(|idx| self.vals[idx] - rhs.vals[idx])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { vals }
    }
}

impl Vector {
    /// Computes the l1-norm of the vector.
    fn norm(&self) -> u32 {
        self.vals.iter().map(|val| val.unsigned_abs()).sum()
    }
}

struct Particle {
    pos: Vector, // position
    vel: Vector, // velocity
    acc: Vector, // acceleration
}

impl From<&str> for Particle {
    fn from(value: &str) -> Self {
        let split: Vec<_> = value.split(", ").map(|s| &s[2..]).collect();

        Self {
            pos: split[0].into(),
            vel: split[1].into(),
            acc: split[2].into(),
        }
    }
}

impl Particle {
    /// Computes the time of the collision between two particles.
    fn collision(&self, other: &Self) -> Option<u32> {
        // Two particles P = (p, v, a) and P' = (p', v', a') collide at time t
        // iff: 2dp + t * 2dv + t(t+1) * da = 0 (where dx := x - x').
        // This gives three quadratic equations (one per coordinates) that t
        // must satisfy, under the condition that t is an integer >= 1.

        let d_pos = self.pos - other.pos;
        let d_vel = self.vel - other.vel;
        let d_acc = self.acc - other.acc;

        // Store the roots of all three equations and their number of occurences (<= 3).
        let mut roots = HashMap::new();

        // Count the number of nontrivial equations.
        // The collision times are the simultaneous solutions to all nontrivial equations.
        let mut nontrivial_eqs = 3;

        for idx in 0..3 {
            // write the equation as a * t^2 + b * t + c = 0
            let a = d_acc.vals[idx];
            let b = 2 * d_vel.vals[idx] + d_acc.vals[idx];
            let c = 2 * d_pos.vals[idx];

            // ignore the trivial equations (a = b = c = 0)
            if a | b | c == 0 {
                nontrivial_eqs -= 1;
                continue;
            }

            // solve the quadratic equation and insert the roots into the map
            for root in solve_quadratic_equation(a, b, c).into_iter() {
                *roots.entry(root).or_insert(0) += 1;
            }
        }

        // find the smallest t in the map which is solution to all nontrivial equations
        roots
            .into_iter()
            .filter_map(|(t, count)| {
                if count == nontrivial_eqs {
                    Some(t)
                } else {
                    None
                }
            })
            .min()
    }
}

/// Solves the quadratic equation ax^2 + bx + c = 0.
///
/// Returns the non-negative integer roots.
///
/// This function assumes that the equation is not trivial, i.e.
/// that one of a, b, or c is non-zero.
fn solve_quadratic_equation(a: i32, b: i32, c: i32) -> Vec<u32> {
    if a == 0 {
        if (b == 0) || (c % b != 0) || (b * c > 0) {
            return Vec::new();
        }

        return Vec::from([(-c / b) as u32]);
    }

    let (a, b, c) = (a as f32, b as f32, c as f32);
    let delta = b * b - 4.0 * a * c;

    // retain only the positive integers roots
    let roots: HashSet<_> = Vec::from([
        (-b + delta.sqrt()) / (2.0 * a),
        (-b - delta.sqrt()) / (2.0 * a),
    ])
    .into_iter()
    .filter(|&root| root >= 0.0)
    .filter_map(|root| match root.fract() {
        0.0 => Some(root as u32),
        _ => None,
    })
    .collect();

    roots.into_iter().collect()
}

// The value of the acceleration, velocity, and position
// of a particle at time t (i.e. after t steps) are
// * a(t) = a(0)
// * v(t) = v(0) + t * a(0)
// * p(t) = p(0) + t * v(0) + t(t+1)/2 * a(0)

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let particles: Vec<_> = input.lines().map(Particle::from).collect();

    // --- Part One --- //

    // Because the position grows quadratically with the acceleration,
    // the particle closest to the origin in the long term is the particle
    // with the lowest acceleration (as it gets passed by every other particle).

    // find the index of the particle with the lowest acceleration
    let part_one = particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, part)| part.acc.norm())
        .unwrap()
        .0;

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    // list the remaining particles (after collisions)
    let mut remaining: Vec<_> = (0..particles.len()).map(|_| true).collect();

    // store all collisions in a map: time -> list of collisions (pairs of indices)
    let mut collisions = HashMap::new();

    for idx1 in 0..particles.len() {
        for idx2 in (idx1 + 1)..particles.len() {
            if let Some(time) = particles[idx1].collision(&particles[idx2]) {
                collisions
                    .entry(time)
                    .or_insert(Vec::new())
                    .push((idx1, idx2));
            }
        }
    }

    // list the times at which collisions happen in chronological order
    let mut collision_times: Vec<_> = collisions.keys().collect();
    collision_times.sort();

    // go through the collision times and delete any colliding particles
    for time in collision_times {
        let mut delete = HashSet::new();

        for &(idx1, idx2) in collisions.get(time).unwrap() {
            // if both particles are still present at time t-1, mark them for deletion
            if remaining[idx1] & remaining[idx2] {
                delete.insert(idx1);
                delete.insert(idx2);
            }
        }

        // remove all particles involved in a collision at time t
        for idx in delete {
            remaining[idx] = false;
        }
    }

    let part_two = remaining.into_iter().filter(|&rem| rem).count();

    println!("Part Two: {}", part_two);
}
