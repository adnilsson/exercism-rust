use std::collections::HashMap;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    // a minefield is guaranteed to be ASCII, so its safe to treat characters as u8
    let mut ascii_minefield: Vec<Vec<u8>> =
        minefield.iter().map(|&s| s.as_bytes().to_vec()).collect();

    let mut mine_counter: HashMap<(_, _), u8> = HashMap::new();
    for (i, &row) in minefield.iter().enumerate() {
        let mine_positions = find_mines_linear(row.as_bytes());
        for j in mine_positions {
            let pos = neighborhood((i, j));
            for mine_pos in pos {
                mine_counter
                    .entry(mine_pos)
                    .and_modify(|mine_count| *mine_count += 1)
                    .or_insert(1);
            }
        }
    }

    for (i, row) in minefield.iter().enumerate() {
        let open_cells = row.char_indices().filter(|(_, c)| c.is_whitespace());
        for (j, _) in open_cells {
            if let Some(count) = mine_counter.get(&(i, j)) {
                let elem: &mut u8 = ascii_minefield.get_mut(i).unwrap().get_mut(j).unwrap();
                *elem = digit_to_ascii_codepoint(*count).unwrap();
            }
        }
    }

    ascii_minefield
        .into_iter()
        .map(|s| String::from_utf8(s).unwrap())
        .collect()
}

fn find_mines_linear(mine_line: &[u8]) -> Vec<usize> {
    mine_line
        .iter()
        .enumerate()
        .filter(|c| *c.1 == b'*')
        .map(|(positions, _)| positions)
        .collect()
}

fn neighborhood(coordinate: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![
        (coordinate.0 + 1, coordinate.1),
        (coordinate.0 + 1, coordinate.1 + 1),
        (coordinate.0, coordinate.1 + 1),
    ];

    if coordinate.0 > 0 {
        neighbors.extend_from_slice(&[
            (coordinate.0 - 1, coordinate.1),
            (coordinate.0 - 1, coordinate.1 + 1),
        ]);
    }
    if coordinate.1 > 0 {
        neighbors.extend_from_slice(&[
            (coordinate.0, coordinate.1 - 1),
            (coordinate.0 + 1, coordinate.1 - 1),
        ]);
    }
    if coordinate.0 > 0 && coordinate.1 > 0 {
        neighbors.push((coordinate.0 - 1, coordinate.1 - 1))
    }

    neighbors
}

fn digit_to_ascii_codepoint(i: u8) -> Option<u8> {
    if i > 9 {
        return None;
    }
    Some(i.to_string().as_bytes()[0])
}
