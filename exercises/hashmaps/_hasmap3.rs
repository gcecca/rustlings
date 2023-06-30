use std::collections::HashMap;

// A structure to store team name and its goal details.
#[derive(Debug)]
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn update_or_insert(scores: &mut HashMap<String, Team>, team_name: &str,
		    team: &Team){

    if scores.contains_key(team_name){
	let t = scores.get(team_name);
	let team_status: &Team = match t{
	    Some(t) => t,
	    None => team
	};

	let team_status = Team {
	    name: team_name.to_string(),
	    goals_scored: team.goals_scored + team_status.goals_scored,
	    goals_conceded: team.goals_conceded + team_status.goals_conceded,
	};

	scores.insert(team_name.to_string(), team_status);
    }
    else {
	let team_status = Team {
	    name: team.name.to_string(),
	    goals_scored: team.goals_scored,
	    goals_conceded: team.goals_conceded

	};
	scores.insert(team_name.to_string(), team_status);
    }
}


fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
	let mut team_1 = Team {name: String::from(&team_1_name),
			       goals_scored: team_1_score,
			       goals_conceded: team_2_score};
	
	let mut team_2 = Team {name: String::from(&team_2_name),
			       goals_scored: team_2_score,
			       goals_conceded: team_1_score};

	update_or_insert(&mut scores, &team_1_name, &team_1);
	update_or_insert(&mut scores, &team_2_name, &team_2);
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scoress = build_scores_table(get_results());

        let mut keys: Vec<&String> = scoress.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_scores_1() {
        let scoress = build_scores_table(get_results());
        let team = scoress.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_scores_2() {
        let scoress = build_scores_table(get_results());
        let team = scoress.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
