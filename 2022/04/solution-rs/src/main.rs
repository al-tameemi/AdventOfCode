use std::fs;

struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    fn new(assignment_range: &str) -> Assignment {
        let assignments: Vec<&str> = assignment_range.split('-').collect();
        Assignment {
            start: assignments[0].parse().unwrap(), 
            end: assignments[1].parse().unwrap() 
        }
    }

    fn contains_or_contained(&self, assignment: &Assignment) -> bool {
        (self.start <= assignment.start && self.end >= assignment.end) || 
        (assignment.start <= self.start && assignment.end >= self.end)
    }
    
    fn overlaps(&self, assignment: &Assignment) -> bool {
        self.start <= assignment.end && self.end >= assignment.start
    }
}

fn main() {
    let binding = fs::read_to_string("input").unwrap();
    let data = binding.split("\n");
    
    let total_contained = data.clone().map(|pairs| {
                                        let pairs = pairs.split(",").map(Assignment::new).collect::<Vec<Assignment>>();
                                        pairs[0].contains_or_contained(&pairs[1]) as i32
                                    })
                                    .sum::<i32>();
    println!("{total_contained}");

    let total_overlap = data.clone().map(|pairs| {
                                    let pairs = pairs.split(",").map(Assignment::new).collect::<Vec<Assignment>>();
                                    pairs[0].overlaps(&pairs[1]) as i32
                                })
                                .sum::<i32>();
    println!("{total_overlap}");
}
