pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>>;

        match row_count {
            0 => triangle = vec![],
            1 => triangle = vec![vec![1]],
            _ => {
                triangle = vec![vec![1], vec![1, 1]];

                for floor in 3..=row_count {
                    let mut current_row = vec![1];

                    let previous_row = &triangle.last().unwrap();

                    for index in 1..=floor - 2 {
                        current_row.push(previous_row[index as usize] + previous_row[(index - 1) as usize]);
                    }

                    current_row.push(1);

                    triangle.push(current_row);
                }
            }
        }

        Self { triangle: triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}