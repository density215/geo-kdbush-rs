use kdbush::kdbush::{KDBush, RawCoord};

fn main() {
    let points = vec![
        (54, 1),
        (97, 21),
        (65, 35),
        (33, 54),
        (95, 39),
        (54, 3),
        (53, 54),
        (84, 72),
        (33, 34),
        (43, 15),
        (52, 83),
        (81, 23),
        (1, 61),
        (38, 74),
        (11, 91),
        (24, 56),
        (90, 31),
        (25, 57),
        (46, 61),
        (29, 69),
        (49, 60),
        (4, 98),
        (71, 15),
        (60, 25),
        (38, 84),
        (52, 38),
        (94, 51),
        (13, 25),
        (77, 73),
        (88, 87),
        (6, 27),
        (58, 22),
        (53, 28),
        (27, 91),
        (96, 98),
        (93, 14),
        (22, 93),
        (45, 94),
        (18, 28),
        (35, 15),
        (19, 81),
        (20, 81),
        (67, 53),
        (43, 3),
        (47, 66),
        (48, 34),
        (46, 12),
        (32, 38),
        (43, 12),
        (39, 94),
        (88, 62),
        (66, 14),
        (84, 30),
        (72, 81),
        (41, 92),
        (26, 4),
        (6, 76),
        (47, 21),
        (57, 70),
        (71, 82),
        (50, 68),
        (96, 18),
        (40, 31),
        (78, 53),
        (71, 90),
        (32, 14),
        (55, 6),
        (32, 88),
        (62, 32),
        (21, 67),
        (73, 81),
        (44, 64),
        (29, 50),
        (70, 5),
        (6, 22),
        (68, 3),
        (11, 23),
        (20, 42),
        (21, 73),
        (63, 86),
        (9, 40),
        (99, 2),
        (99, 76),
        (56, 77),
        (83, 6),
        (21, 72),
        (78, 30),
        (75, 53),
        (41, 11),
        (95, 20),
        (30, 38),
        (96, 82),
        (65, 48),
        (33, 18),
        (87, 28),
        (10, 10),
        (40, 34),
        (10, 20),
        (47, 29),
        (46, 78),
    ];

    let kdb = KDBush::new(
        points.iter().map(|p| RawCoord(p.0, p.1)).collect(),
        10,
    );

    let mut range_idx = vec![];

    let sorted_kdb = kdb.unwrap();

    println!("{:?}", sorted_kdb);

    &sorted_kdb.within(50, 50, 20, &mut range_idx, None, None, None);

    println!("{:?}", range_idx);
}

#[cfg(test)]
mod tests {
    use kdbush::kdbush::{RawCoord};

    fn get_points() -> Vec<RawCoord<i16>> {
        vec![
            (54, 1),
            (97, 21),
            (65, 35),
            (33, 54),
            (95, 39),
            (54, 3),
            (53, 54),
            (84, 72),
            (33, 34),
            (43, 15),
            (52, 83),
            (81, 23),
            (1, 61),
            (38, 74),
            (11, 91),
            (24, 56),
            (90, 31),
            (25, 57),
            (46, 61),
            (29, 69),
            (49, 60),
            (4, 98),
            (71, 15),
            (60, 25),
            (38, 84),
            (52, 38),
            (94, 51),
            (13, 25),
            (77, 73),
            (88, 87),
            (6, 27),
            (58, 22),
            (53, 28),
            (27, 91),
            (96, 98),
            (93, 14),
            (22, 93),
            (45, 94),
            (18, 28),
            (35, 15),
            (19, 81),
            (20, 81),
            (67, 53),
            (43, 3),
            (47, 66),
            (48, 34),
            (46, 12),
            (32, 38),
            (43, 12),
            (39, 94),
            (88, 62),
            (66, 14),
            (84, 30),
            (72, 81),
            (41, 92),
            (26, 4),
            (6, 76),
            (47, 21),
            (57, 70),
            (71, 82),
            (50, 68),
            (96, 18),
            (40, 31),
            (78, 53),
            (71, 90),
            (32, 14),
            (55, 6),
            (32, 88),
            (62, 32),
            (21, 67),
            (73, 81),
            (44, 64),
            (29, 50),
            (70, 5),
            (6, 22),
            (68, 3),
            (11, 23),
            (20, 42),
            (21, 73),
            (63, 86),
            (9, 40),
            (99, 2),
            (99, 76),
            (56, 77),
            (83, 6),
            (21, 72),
            (78, 30),
            (75, 53),
            (41, 11),
            (95, 20),
            (30, 38),
            (96, 82),
            (65, 48),
            (33, 18),
            (87, 28),
            (10, 10),
            (40, 34),
            (10, 20),
            (47, 29),
            (46, 78),
        ].iter()
        .map(|p| RawCoord(p.0, p.1))
        .collect()
    }

    #[test]
    fn test_sort() {
        let points = get_points();

        let ids: Vec<usize> = vec![
            97, 74, 95, 30, 77, 38, 76, 27, 80, 55, 72, 90, 88, 48, 43, 46, 65, 39, 62, 93, 9, 96,
            47, 8, 3, 12, 15, 14, 21, 41, 36, 40, 69, 56, 85, 78, 17, 71, 44, 19, 18, 13, 99, 24,
            67, 33, 37, 49, 54, 57, 98, 45, 23, 31, 66, 68, 0, 32, 5, 51, 75, 73, 84, 35, 81, 22,
            61, 89, 1, 11, 86, 52, 94, 16, 2, 6, 25, 92, 42, 20, 60, 58, 83, 79, 64, 10, 59, 53,
            26, 87, 4, 63, 50, 7, 28, 82, 70, 29, 34, 91,
        ];

        let sorted_kdb =
            kdbush::kdbush::KDBush::new(points, 10);

        assert_eq!(sorted_kdb.unwrap().ids, ids);
    }

    #[test]
    fn test_range() {
        let points = get_points();
        let expected_ids = [
            3, 90, 77, 72, 62, 96, 47, 8, 17, 15, 69, 71, 44, 19, 18, 45, 60, 20,
        ];

        let sorted_kdb =
            kdbush::kdbush::KDBush::new(points, 10)
                .unwrap();
        let mut range_ids = vec![];
        &sorted_kdb.range(&20, &30, &50, &70, &mut range_ids, None, None, None);
        println!("{:?}", range_ids);
        println!("{:?}", expected_ids);
        assert_eq!(range_ids, expected_ids);
    }

    #[test]
    fn test_radius() {
        let points = get_points();
        let expected_ids = [3, 96, 71, 44, 18, 45, 60, 6, 25, 92, 42, 20];
        let sorted_kdb =
            kdbush::kdbush::KDBush::new(points, 10)
                .unwrap();
        let mut within_ids = vec![];
        &sorted_kdb.within(50, 50, 20, &mut within_ids, None, None, None);
        assert_eq!(within_ids, expected_ids);
    }

    #[test]
    fn test_empty() {
        let points: Vec<RawCoord<i16>> = vec![];
        let mut range_ids = vec![];
        let sorted_kdb =
            kdbush::kdbush::KDBush::new(points, 10)
                .unwrap();
        &sorted_kdb.range(&20, &30, &50, &70, &mut range_ids, None, None, None);
        println!("{:?}", sorted_kdb);
        assert_eq!(range_ids.is_empty(), true);

        let mut within_ids = vec![];
        &sorted_kdb.within(50, 50, 20, &mut within_ids, None, None, None);
        assert_eq!(within_ids.is_empty(), true);
    }
}
