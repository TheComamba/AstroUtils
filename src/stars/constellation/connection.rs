use crate::{stars::star_appearance::StarAppearance, units::angle::FULL_CIRC};
use simple_si_units::geometry::Angle;
use std::cmp::Ordering;

pub struct Connection {
    from: usize,
    to: usize,
    distance: Angle<f64>,
}

impl Connection {
    fn new(from: usize, to: usize, stars: &[StarAppearance]) -> Self {
        let distance = stars[from].get_pos().angle_to(stars[to].get_pos());
        Connection { from, to, distance }
    }

    pub fn get_indices(&self) -> (usize, usize) {
        (self.from, self.to)
    }

    fn connects_to(&self, i: usize) -> bool {
        self.from == i || self.to == i
    }

    fn other_end(&self, i: usize) -> usize {
        if self.from == i {
            self.to
        } else {
            self.from
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

fn shortest_path(start: usize, end: usize, connections: &[Connection]) -> Option<Vec<&Connection>> {
    let mut paths: Vec<Vec<&Connection>> = Vec::new();
    for connection in connections {
        if connection.connects_to(start) {
            let mut path = vec![connection];
            if connection.connects_to(end) {
                return Some(path);
            } else if let Some(mut sub_path) =
                shortest_path(connection.other_end(start), end, connections)
            {
                path.append(&mut sub_path);
                paths.push(path);
            }
        }
    }
    if paths.is_empty() {
        return None;
    }
    paths.sort_by(|a, b| a.len().cmp(&b.len()));
    Some(paths[0].clone())
}

fn is_reachable_within(
    start: usize,
    end: usize,
    max_steps: usize,
    connections: &[Connection],
) -> bool {
    if max_steps == 0 && start != end {
        return false;
    }
    for connection in connections {
        if connection.connects_to(start) {
            if connection.connects_to(end) {
                return true;
            } else if is_reachable_within(
                connection.other_end(start),
                end,
                max_steps - 1,
                connections,
            ) {
                return true;
            }
        }
    }
    false
}

fn connections_sorted_by_distance(stars: &[StarAppearance]) -> Vec<Vec<Connection>> {
    let mut connections: Vec<Vec<Connection>> = Vec::new();
    for i in 0..stars.len() {
        let mut star_connections: Vec<Connection> = Vec::new();
        for j in 0..stars.len() {
            if i != j {
                star_connections.push(Connection::new(i, j, stars));
            }
        }
        star_connections.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(Ordering::Equal)
        });
        connections.push(star_connections);
    }
    connections
}

fn sorted_connections(stars: &[StarAppearance]) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            connections.push(Connection::new(i, j, stars));
        }
    }
    connections.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(Ordering::Equal)
    });
    connections
}

fn nearest_neighbours(i: usize, stars: &[StarAppearance]) -> Vec<usize> {
    let mut neighbours: Vec<usize> = Vec::new();
    for j in 0..stars.len() {
        if i != j {
            neighbours.push(j);
        }
    }
    neighbours.sort_by(|a, b| {
        stars[i]
            .get_pos()
            .angle_to(stars[*a].get_pos())
            .partial_cmp(&stars[i].get_pos().angle_to(stars[*b].get_pos()))
            .unwrap_or(Ordering::Equal)
    });
    neighbours
}

fn all_nearest_neighbours(stars: &[StarAppearance]) -> Vec<Vec<usize>> {
    let mut all_neighbours: Vec<Vec<usize>> = Vec::new();
    for i in 0..stars.len() {
        all_neighbours.push(nearest_neighbours(i, stars));
    }
    all_neighbours
}

fn get_max_allowed_steps(end: usize, nearest_neighbours: &Vec<usize>) -> usize {
    nearest_neighbours
        .iter()
        .position(|&i| i == end)
        .unwrap_or(0)
        + 1
}

pub(super) fn collect_connections(stars: &[StarAppearance]) -> Vec<Connection> {
    let all_nearest_neighbours = all_nearest_neighbours(stars);
    let all_connections = sorted_connections(stars);
    let mut connections: Vec<Connection> = Vec::new();
    for connection in all_connections {
        if connections.contains(&connection) {
            continue;
        }

        let start = connection.get_indices().0;
        let end = connection.get_indices().1;

        let max_steps =
            get_max_allowed_steps(end, &all_nearest_neighbours[connection.get_indices().0]);
        if is_reachable_within(start, end, max_steps, &connections) {
            connections.push(connection);
        }
    }
    connections
}

fn find_nearest_neighbour(
    index: usize,
    stars: &[StarAppearance],
    excluding: &Vec<usize>,
) -> Option<usize> {
    let mut nearest_neighbour = None;
    let pos = stars[index].get_pos();
    for j in 0..stars.len() {
        if index != j && !excluding.contains(&j) {
            let distance = stars[j].get_pos().angle_to(pos);
            if let Some(nn) = nearest_neighbour {
                let nn_distance = stars[nn as usize].get_pos().angle_to(pos);
                if distance < nn_distance {
                    nearest_neighbour = Some(j);
                }
            } else {
                nearest_neighbour = Some(j);
            }
        }
    }
    nearest_neighbour
}

fn minimum_spanning_tree(stars: &[StarAppearance]) -> Vec<Connection> {
    // This is Prim's algorithm
    let mut connections = Vec::new();
    if stars.len() < 2 {
        return connections;
    }
    let mut visited = vec![0];
    while visited.len() < stars.len() {
        let mut current_best = Connection {
            to: 0,
            from: 0,
            distance: FULL_CIRC,
        };
        for i in &visited {
            let nn = find_nearest_neighbour(*i, stars, &visited);
            if let Some(nn) = nn {
                let connection = Connection::new(*i, nn, stars);
                if connection.distance < current_best.distance {
                    current_best = connection;
                }
            }
        }
        visited.push(current_best.to);
        connections.push(current_best);
    }
    connections
}

#[cfg(test)]
mod tests {
    use simple_si_units::electromagnetic::Illuminance;

    use super::*;
    use crate::{
        color::sRGBColor, coordinates::spherical::SphericalCoordinates,
        real_data::stars::BRIGHTEST_STARS,
        stars::constellation::constellation::collect_constellations, units::angle::ANGLE_ZERO,
    };

    fn stars_in_line(size: usize) -> Vec<StarAppearance> {
        let mut stars = Vec::new();
        for i in 0..size {
            // Making distances distinct
            let longitude = Angle::from_degrees(10. * i as f64 + (i as f64).powi(2) / 100.);
            assert!(longitude.to_degrees() < 179.0);
            let pos = SphericalCoordinates::new(longitude, ANGLE_ZERO).to_ecliptic();
            stars.push(StarAppearance::new(
                format!("Star {}", i),
                Illuminance::from_lux(1.0),
                sRGBColor::DEFAULT,
                pos,
            ));
        }
        stars
    }

    fn connections_in_line(size: usize) -> Vec<Connection> {
        let mut connections = Vec::new();
        for i in 0..size {
            connections.push(Connection {
                from: i,
                to: i + 1,
                distance: ANGLE_ZERO,
            });
        }
        connections
    }

    #[test]
    fn nearest_neighbours_of_line_are_sorted() {
        let size = 10;
        let stars = stars_in_line(size);
        let neighbours = nearest_neighbours(0, &stars);
        assert!(neighbours.len() == size - 1);
        for i in 1..size {
            assert!(neighbours[i - 1] == i);
        }

        let nearest_neighbours = nearest_neighbours(size - 1, &stars);
        assert!(nearest_neighbours.len() == size - 1);
        for i in 1..size {
            assert!(nearest_neighbours[i - 1] == size - 1 - i);
        }
    }

    #[test]
    fn all_nearest_neighbours_for_short_line() {
        let stars = stars_in_line(3);
        let all_neighbours = all_nearest_neighbours(&stars);
        let expected = vec![vec![1, 2], vec![0, 2], vec![1, 0]];
        assert!(all_neighbours == expected);
    }

    #[test]
    fn is_reachable() {
        let size = 10;
        let connections = connections_in_line(size);
        for i in 0..size {
            assert!(is_reachable_within(0, i, i, &connections));
            assert!(!is_reachable_within(0, i + 1, i, &connections));
        }
    }

    #[test]
    fn collect_connections_for_line() {
        for size in 1..10 {
            let stars = stars_in_line(size);
            let connections = collect_connections(&stars);
            assert!(connections.len() == size - 1);
            for i in 0..size - 1 {
                assert!(connections[i].get_indices() == (i, i + 1));
            }
        }
    }

    #[test]
    fn minimum_spanning_tree_has_length_n_minus_1() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let all_consteallations = collect_constellations(&all_stars[..]);
        for constellation in all_consteallations {
            let mst = minimum_spanning_tree(&constellation.get_stars());
            assert!(mst.len() == constellation.get_stars().len() - 1);
        }
    }

    #[test]
    fn minimum_spanning_tree_is_contained_in_constellation_connections() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let all_consteallations = collect_constellations(&all_stars[..]);
        for constellation in all_consteallations {
            let connections = constellation.get_connections();
            let mst = minimum_spanning_tree(&constellation.get_stars());
            for mst_connection in mst {
                assert!(connections.contains(&mst_connection));
            }
        }
    }

    #[test]
    fn constellation_connection_is_independent_of_order() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let all_consteallations = collect_constellations(&all_stars[..]);
        for consetllation in all_consteallations {
            let connections = consetllation.get_connections();
            let mut stars_rev = consetllation.get_stars().clone();
            stars_rev.reverse();
            let connections_rev = collect_connections(&stars_rev);

            assert!(connections.len() == connections_rev.len());
            for connection in connections {
                assert!(connections_rev.contains(&connection));
            }
        }
    }
}