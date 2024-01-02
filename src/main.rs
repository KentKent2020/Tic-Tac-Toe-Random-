use rand::Rng;

#[derive(Debug)]
enum Victory {
    Row1,
    Row2,
    Row3,
    Column1,
    Column2,
    Column3,
    Diagonal1,
    Diagonal2,
}

fn printgrille(grille: [[char; 3]; 3]) {
    println!("\n");
    let char0 = grille[0][0];
    let char1 = grille[0][1];
    let char2 = grille[0][2];
    let char3 = grille[1][0];
    let char4 = grille[1][1];
    let char5 = grille[1][2];
    let char6 = grille[2][0];
    let char7 = grille[2][1];
    let char8 = grille[2][2];



    println!("{}|{}|{}", 
    if char0 == '#' {"\x1b[30m#\x1b[0m"} else if char0 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"},
    if char1 == '#' {"\x1b[30m#\x1b[0m"} else if char1 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"},
    if char2 == '#' {"\x1b[30m#\x1b[0m"} else if char2 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"});


    println!("{}|{}|{}", 
    if char3 == '#' {"\x1b[30m#\x1b[0m"} else if char3 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"},
    if char4 == '#' {"\x1b[30m#\x1b[0m"} else if char4 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"},
    if char5 == '#' {"\x1b[30m#\x1b[0m"} else if char5 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"});


    println!("{}|{}|{}", 
    if char6 == '#' {"\x1b[30m#\x1b[0m"} else if char6 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"},
    if char7 == '#' {"\x1b[30m#\x1b[0m"} else if char7 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"},
    if char8 == '#' {"\x1b[30m#\x1b[0m"} else if char8 == 'X' {"\x1b[31mX\x1b[0m"} else {"\x1b[34m0\x1b[0m"});


    


    // for i in grille.iter() {
    //     println!("{:?}", i);
    // }
}

fn add_to_grid(grille: &mut [[char; 3]; 3], num: usize, character: char) -> bool {
    let row = num / 3;
    let col = num % 3;

    println!("row: {} et col {}", row, col);

    if grille[row][col] != '#' {
        false
    } else {
        grille[row][col] = character;
        true
    }
}

fn checkvictory(grille: &[[char; 3]; 3], character: char) -> Option<Victory> {
    if grille[0][0] == grille[1][0] && grille[1][0] == grille[2][0] && grille[0][0] == character {
        return Some(Victory::Column1);
    } else if grille[0][1] == grille[1][1] && grille[1][1] == grille[2][1] && grille[0][1] == character{
        return Some(Victory::Column2);
    } else if grille[0][2] == grille[1][2] && grille[1][2] == grille[2][2] && grille[0][2] == character {
        return Some(Victory::Column3);
    }

    if grille[0][0] == grille[0][1] && grille[0][1] == grille[0][2] && grille[0][0] == character{
        return Some(Victory::Row1);
    } else if grille[1][0] == grille[1][1] && grille[1][1] == grille[1][2]&& grille[1][0] == character {
        return Some(Victory::Row2);
    } else if grille[2][0] == grille[2][1] && grille[2][1] == grille[2][2] && grille[2][0] == character {
        return Some(Victory::Row3);
    }

    if grille[0][0] == grille[1][1] && grille[1][1] == grille[2][2] && grille[0][0] == character {
        return Some(Victory::Diagonal1);
    } else if grille[0][2] == grille[1][1] && grille[1][1] == grille[2][0]&& grille[0][2] == character {
        return Some(Victory::Diagonal2);
    }

    None
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut grille = [
        ['#', '#', '#'],
        ['#', '#', '#'],
        ['#', '#', '#'],
    ];

    printgrille(grille);
    
    let firstplayer = ['X', 'O'];
    

    for i in 0..=50 {

        let first_player_to_play = if let 0 = i % 2{
            'X'
        }
        else{
            'O'
        };
        

        let randomnumber = rng.gen_range(0..=8);

        if !add_to_grid(&mut grille, randomnumber, first_player_to_play) {
            println!("Impossible de mettre {} à l'index {}", first_player_to_play,randomnumber);
        }

        printgrille(grille);

        if let Some(victory) = checkvictory(&grille, first_player_to_play) {
            println!("Victoire: {:?}", victory);
            break;
        }
    }
}
