mod drunk;

fn main() {
    // 200 rows; 130 columns
    let mut map = [['#'; 130]; 200];

    drunk::drunkards(&mut map);
    display(map);
}

fn display(map: [[char; 130]; 200]) {
    for y in 0..200 {
        for x in 0..130 {
            print!("{}", map[y][x]);
        }
        print!("\n");
    }
}
