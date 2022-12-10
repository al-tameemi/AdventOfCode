#[derive(Debug, Clone, PartialEq)]
enum Visibility {
    Visible,
    NotVisible,
}

#[derive(Clone, Debug)]
struct Tree {
    height: u32,
    visible: Visibility,
    score: u32,
}

fn main() {
    let file = std::fs::read_to_string("input").unwrap();

    let height = file.lines().count();
    let width = file.lines().nth(0).unwrap().len();

    let mut trees: Vec<Vec<Tree>> = vec![
        vec![
            Tree {
                height: 0,
                visible: Visibility::Visible,
                score: 0,
            };
            width
        ];
        height
    ];

    file.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, character)| {
            trees[x][y].height = character.to_digit(10).unwrap();
        });
    });

    //
    //                 (x, y)
    //
    //                 (0,0)     (1,0) (2,0) (3,0)      (4,0)
    //                             2     2     2
    //                             |     |     |
    //                             V     V     V
    //                 (0,1) 1-> (1,1) (2,1) (3,1) <-3  (4,1)
    //                 (0,2) 2-> (1,2) (2,2) (3,2) <-3  (4,2)
    //                 (0,3) 3-> (1,3) (2,3) (3,3) <-3  (4,3)
    //                             ^     ^     ^
    //                             |     |     |
    //                             4     4     4
    //                 (0,4)     (1,4) (2,4) (3,4)      (4,4)
    //
    let mut visible_count = height * width;
    let mut max_score = 0;
    let mut not_vis_count = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // println!("{x}");
            // println!("({x},{y}): {}", trees[x][y].height);
            // check left
            let mut not_vis_count = 0;
            let mut score = 1;
            let mut temp_score = 0;
            for x_alt in (0..x).rev() {
                temp_score += 1;
                if trees[x_alt][y].height >= trees[x][y].height {
                    not_vis_count += 1;
                    break;
                }
            }
            score = score * temp_score;
            println!("left: {temp_score}");
        
            let mut temp_score = 0;
            // check right
            for x_alt in x + 1..width {
                temp_score += 1;
                if trees[x_alt][y].height >= trees[x][y].height {
                    not_vis_count += 1;
                    break;
                }
            }
            score = score * temp_score;
            println!("right: {temp_score}");
        
            let mut temp_score = 0;
            // check up
            for y_alt in (0..y).rev() {
                temp_score += 1;
                if trees[x][y_alt].height >= trees[x][y].height {
                    not_vis_count += 1;
                    break;
                }
            }
            score = score * temp_score;
            println!("up: {temp_score}");
        
            let mut temp_score = 0;
            // check down
            for y_alt in y + 1..height {
                temp_score += 1;
                if trees[x][y_alt].height >= trees[x][y].height {
                    not_vis_count += 1;
                    break;
                }
            }
            score = score * temp_score;
            println!("down: {temp_score}");
        
            if not_vis_count == 4 {
                trees[x][y].visible = Visibility::NotVisible;
                visible_count -= 1;
            }
        
            if score > max_score {
                max_score = score;
            }
        
            trees[x][y].score = score;
        }
        // break;
    }
    // println!("{:#?}", trees);
    println!("{visible_count}");
    println!("{max_score}");
}
