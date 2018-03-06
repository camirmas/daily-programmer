fn main() {
    calc_world_domination(2, 4, 1000000000);
}

fn sum_fertile(rabbits: &Vec<usize>) -> usize {
    let mut count = 0;
    for r in &rabbits[4..] {
        count += r;
    }
    count
}

fn sum_rabbits(rabbits: &Vec<usize>) -> usize {
    let mut count = 0;
    for r in rabbits {
        count += r;
    }
    count
}

fn calc_world_domination(male: usize, female: usize, needed_alive: usize) -> usize {
    let mut m_rabbits = Vec::new();
    let mut f_rabbits = Vec::new();
    let mut months = 0;

    for _ in 0..97 {
        m_rabbits.push(0);
        f_rabbits.push(0);
    }

    m_rabbits[2] = male;
    f_rabbits[2] = female;

    while (sum_rabbits(&m_rabbits) + sum_rabbits(&f_rabbits)) < needed_alive {
        let new_b = sum_fertile(&f_rabbits);
        let bun_f = new_b * 9;
        let bun_m = new_b * 5;

        m_rabbits = m_rabbits[0..96].to_vec();
        f_rabbits = f_rabbits[0..96].to_vec();

        m_rabbits.insert(0, bun_m);
        f_rabbits.insert(0, bun_f);

        months += 1;
    }
    months
}

#[test]
fn test_calc_world_domination() {
    assert_eq!(calc_world_domination(2, 4, 1000000000), 32);
    assert_eq!(calc_world_domination(2, 4, 15000000000), 36);
}
