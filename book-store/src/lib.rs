use itertools::Itertools;

const UNIT_PRICE: u32 = 800;
const DISCOUNTS: [u32; 6] = [0, 0, 5, 10, 20, 25];

fn calc_price(count: usize) -> u32 {
    let price = (count as u32) * UNIT_PRICE;
    price * (100 - DISCOUNTS[count]) / 100
}

struct Booksets {
    books: Vec<usize>,
}

impl Booksets {
    fn new(books: &[u32]) -> Self {
        let mut books = Vec::from(books);
        books.sort();
        Booksets {
            books: books
                .iter()
                .group_by(|&x| x)
                .into_iter()
                .map(|(_, g)| g.count())
                .collect(),
        }
    }

    fn calc_price(&self) -> u32 {
        let mut counts = Vec::from(&self.books[..]);
        counts.sort();

        // two groups of four
        if counts.len() == 5 && counts[2] != counts[1] {
            // let take all double fours
            let n = (counts[2] / 2).min(counts[1]).min(counts[0]);
            2 * n as u32 * calc_price(4)
        } else {
            // otherwise take larger set first
            let mut price = 0;
            let mut used = 0;
            for (i, x) in counts.iter().enumerate() {
                let n = x - used;
                price += calc_price(counts.len() - i) * n as u32;
                used += n;
            }
            price
        }
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let bs = Booksets::new(books);
    bs.calc_price()
}
