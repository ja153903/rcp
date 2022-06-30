#![allow(dead_code)]
struct Solution;

struct Node {
    position: i32,
    speed: i32,
}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // this question seems like a variant of a monotonic stack
        // One approach we can initiall take is to sort the position and speed
        // by position in reverse order.
        // this way we can take care of the cars that are already close to the target
        let mut nodes: Vec<Node> = Vec::new();
        let mut stack: Vec<f32> = Vec::new();

        for (&pos, &spd) in position.iter().zip(speed.iter()) {
            nodes.push(Node {
                position: pos,
                speed: spd,
            });
        }

        nodes.sort_by_key(|node| node.position);

        for node in nodes.iter().rev() {
            // y = mx + b where y = target, m = speed, and b = position
            // x = (y - b) / m
            let x: f32 = (target - node.position) as f32 / node.speed as f32;
            if stack.is_empty() {
                stack.push(x);
            } else if let Some(&last) = stack.last() {
                if last < x {
                    stack.push(x);
                }
            }
        }

        stack.len() as i32
    }
}
