use std::collections::HashSet;

#[derive(Debug)]
struct Player {
    hits: [[u8; 5]; 5],
    board: Vec<Vec<i32>>,
    winner: bool,
}

impl Player {
    fn mark_number(&mut self, number: i32) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if num == &number {
                    self.hits[i][j] = 1;
                    self.winner = self.check_win_condition(i, j);
                }
            }
        }
    }

    fn check_win_condition(&self, i: usize, j: usize) -> bool {
        let horizontal_sum: u8 = self.hits[i].iter().sum();

        let mut vertical_sum: u8 = 0;
        for x in 0..5 {
            vertical_sum += self.hits[x][j];
        }

        return horizontal_sum == 5 || vertical_sum == 5;
    }

    fn calculate_points(&self) -> i32 {
        let mut sum = 0;
        for (i, row) in self.board.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if self.hits[i][j] == 0 {
                    sum += num;
                }
            }
        }

        return sum;
    }
}

pub fn run(data: Vec<String>) {
    let numbers: Vec<i32> = data[0]
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let players = create_players(&data);

    // find_winner(numbers, players);
    find_last_winner(numbers, players);
}

fn find_last_winner(numbers: Vec<i32>, mut players: Vec<Player>) {
    let mut winners = 0;
    let mut winning_players = HashSet::new();
    let mut last_player = 0;
    let number_of_players = players.len();

    for number in numbers {
        for (i, player) in players.iter_mut().enumerate() {
            if winning_players.contains(&i) {
                continue;
            }

            player.mark_number(number);
            if player.winner {
                winning_players.insert(i);
                winners += 1;
                last_player = i;
            }
        }

        if winners == number_of_players {
            println!(
                "Winner: {:?}",
                players[last_player].calculate_points() * number
            );
            break;
        }
    }
}

fn find_winner(numbers: Vec<i32>, mut players: Vec<Player>) {
    let mut winner = None;
    for number in numbers {
        for (i, player) in players.iter_mut().enumerate() {
            player.mark_number(number);
            if player.winner {
                winner = Some(i);
            }
        }

        if winner.is_some() {
            println!(
                "Winner: {:?}",
                players[winner.unwrap()].calculate_points() * number
            );
            break;
        }
    }
}

fn test_cases(mut players: Vec<Player>) {
    // Horizontal winner
    // players[0].mark_number(22);
    // players[0].mark_number(13);
    // players[0].mark_number(17);
    // players[0].mark_number(11);
    // players[0].mark_number(0);

    // Vertical winner
    // players[0].mark_number(22);
    // players[0].mark_number(8);
    // players[0].mark_number(21);
    // players[0].mark_number(6);
    // players[0].mark_number(1);

    // Test winner
    // players[2].mark_number(14);
    // players[2].mark_number(21);
    // players[2].mark_number(17);
    // players[2].mark_number(24);
    // players[2].mark_number(4);
    // println!("{:?}", players[2]);
}

fn create_players(data: &Vec<String>) -> Vec<Player> {
    let mut player_board: Vec<Vec<i32>> = Vec::new();

    let mut players: Vec<Player> = Vec::new();
    for (i, line) in data.iter().skip(1).enumerate() {
        if i % 6 == 0 {
            if !player_board.is_empty() {
                players.push(Player {
                    hits: [[0; 5]; 5],
                    board: player_board.clone(),
                    winner: false,
                });
                player_board.clear();
            }
            continue;
        }

        // println! {"Index {}: {:?}", i, line};
        let numbers_in_line: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        player_board.push(numbers_in_line)
    }

    players.push(Player {
        hits: [[0; 5]; 5],
        board: player_board.clone(),
        winner: false,
    });

    return players;
}
