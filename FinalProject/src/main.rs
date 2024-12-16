mod analysis;
mod graph;
mod clustering;
#[cfg(test)]
mod test;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self};
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    age: u32,
    nationality: String,
    club: String,
    overall: u32,
    potential: u32,
    best_position: String,
    best_overall_rating: f32,
    stats: HashMap<String, f32>, // Key-value pairs for player attributes
}

struct SoccerData {
    players: HashMap<String, Player>,
    clubs: HashMap<String, HashSet<String>>, // Map of club names to player names
}

impl SoccerData {
    fn new() -> Self {
        SoccerData {
            players: HashMap::new(),
            clubs: HashMap::new(),
        }
    }

    // Add a player to the dataset
    fn add_player(&mut self, player: Player) {
        let player_key = player.name.to_lowercase();
        if let Some(existing_player) = self.players.get(&player_key) {
            // Keep the player with the higher overall score
            if player.overall > existing_player.overall {
                self.players.insert(player_key.clone(), player.clone());
            }
        } else {
            self.players.insert(player_key.clone(), player.clone());
        }

        // Update the club mapping
        let club_name = player.club.to_lowercase();
        self.clubs
            .entry(club_name)
            .or_insert_with(HashSet::new)
            .insert(player_key);
    }

    // Compare two players
    fn compare_players(&self, player1_name: &str, player2_name: &str) {
        let player1 = self.players.get(&player1_name.to_lowercase());
        let player2 = self.players.get(&player2_name.to_lowercase());

        match (player1, player2) {
            (Some(p1), Some(p2)) => {
                println!("Comparison between {} and {}:", p1.name, p2.name);
                println!("Age: {} vs {}", p1.age, p2.age);
                println!("Nationality: {} vs {}", p1.nationality, p2.nationality);
                println!("Club: {} vs {}", p1.club, p2.club);
                println!("Overall: {} vs {}", p1.overall, p2.overall);
                println!("Potential: {} vs {}", p1.potential, p2.potential);
                println!("Best Position: {} vs {}", p1.best_position, p2.best_position);
                println!(
                    "Best Overall Rating: {} vs {}",
                    p1.best_overall_rating, p2.best_overall_rating
                );

                println!("Stats Comparison:");
                for stat in [
                    "Height", "Weight", "Crossing", "Finishing", "HeadingAccuracy",
                    "ShortPassing", "Volleys", "Dribbling", "Curve", "LongPassing",
                    "BallControl", "Acceleration", "SprintSpeed", "Agility", "Reactions",
                    "Balance", "ShotPower", "Jumping", "Stamina", "Strength", "LongShots",
                    "Interceptions", "Positioning", "Vision",
                ] {
                    let p1_value = p1.stats.get(stat).unwrap_or(&0.0);
                    let p2_value = p2.stats.get(stat).unwrap_or(&0.0);
                    println!("{}: {} vs {}", stat, p1_value, p2_value);
                }
            }
            (None, None) => println!("Both players not found."),
            (None, _) => println!("Player {} not found.", player1_name),
            (_, None) => println!("Player {} not found.", player2_name),
        }
    }

    // Find connection path using six degrees of separation
    fn find_connection(&self, player1_name: &str, player2_name: &str) -> Option<Vec<String>> {
        let start = player1_name.to_lowercase();
        let end = player2_name.to_lowercase();

        if !self.players.contains_key(&start) || !self.players.contains_key(&end) {
            return None;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessors = HashMap::new();

        visited.insert(start.clone());
        queue.push_back(start.clone());

        while let Some(current_player) = queue.pop_front() {
            if current_player == end {
                return Some(self.reconstruct_path(&start, &end, &predecessors));
            }

            if let Some(player) = self.players.get(&current_player) {
                if let Some(team_players) = self.clubs.get(&player.club.to_lowercase()) {
                    for teammate in team_players {
                        if !visited.contains(teammate) {
                            visited.insert(teammate.clone());
                            queue.push_back(teammate.clone());
                            predecessors.insert(teammate.clone(), current_player.clone());
                        }
                    }
                }
            }
        }

        None
    }

    fn reconstruct_path(
        &self,
        start: &str,
        end: &str,
        predecessors: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut path = vec![end.to_string()];
        let mut current = end;
        while let Some(predecessor) = predecessors.get(current) {
            path.push(predecessor.clone());
            current = predecessor;
        }
        path.reverse();
        path
    }
}

fn main() -> io::Result<()> {
    let mut soccer_data = SoccerData::new();

    // Load data from the CSV file
    let file_path = "FIFA17_official_data (1).csv";
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path).map_err(|_| {
        eprintln!("Failed to open CSV file");
        std::io::Error::new(std::io::ErrorKind::NotFound, "CSV file not found")
    })?;

    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => {
                eprintln!("Error reading record");
                continue;
            }
        };

        // Extract necessary fields
        let name = record.get(1).unwrap_or("").trim().to_string();
        let age = record.get(2).unwrap_or("0").trim().parse::<u32>().unwrap_or(0);
        let nationality = record.get(4).unwrap_or("").trim().to_string();
        let club = record.get(8).unwrap_or("").trim().to_string();
        let overall = record.get(6).unwrap_or("0").trim().parse::<u32>().unwrap_or(0);
        let potential = record.get(7).unwrap_or("0").trim().parse::<u32>().unwrap_or(0);
        let best_position = record.get(61).unwrap_or("").trim().to_string();
        let best_overall_rating =
            record.get(62).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0);

        let stats = vec![
            ("Height", record.get(25).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Weight", record.get(26).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Crossing", record.get(27).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Finishing", record.get(28).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("HeadingAccuracy", record.get(29).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("ShortPassing", record.get(30).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Volleys", record.get(31).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Dribbling", record.get(32).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Curve", record.get(33).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("LongPassing", record.get(34).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("BallControl", record.get(35).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Acceleration", record.get(36).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("SprintSpeed", record.get(37).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Agility", record.get(38).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Reactions", record.get(39).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Balance", record.get(40).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("ShotPower", record.get(41).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Jumping", record.get(42).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Stamina", record.get(43).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Strength", record.get(44).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("LongShots", record.get(45).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Interceptions", record.get(46).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Positioning", record.get(47).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
            ("Vision", record.get(48).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0)),
        ];

        let stats_map: HashMap<String, f32> = stats.into_iter().map(|(key, value)| (key.to_string(), value)).collect();

        let player = Player {
            name,
            age,
            nationality,
            club,
            overall,
            potential,
            best_position,
            best_overall_rating,
            stats: stats_map,
        };

        soccer_data.add_player(player);
    }

    println!("Enter the first player's name:");
    let mut player1 = String::new();
    io::stdin().read_line(&mut player1).unwrap();
    let player1 = player1.trim();

    println!("Enter the second player's name:");
    let mut player2 = String::new();
    io::stdin().read_line(&mut player2).unwrap();
    let player2 = player2.trim();

    println!("Player Comparison:");
    soccer_data.compare_players(player1, player2);

    println!("Finding connection path...");
    match soccer_data.find_connection(player1, player2) {
        Some(path) => {
            println!("Connection path found:");
            for (i, player) in path.iter().enumerate() {
                println!("{}. {}", i + 1, player);
            }
        }
        None => println!("No connection found between {} and {}", player1, player2),
    }

    Ok(())
}
