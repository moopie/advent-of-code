fn main() {
    println!("AOC 2024 day 05!");

    let file_contents = std::fs::read_to_string("input.txt")
        .expect("err");

    let res = check_rules(&file_contents, false);

    println!("part 1 result: {}", res);

    let res = check_rules(&file_contents, true);

    println!("part 2 result: {}", res);
}

#[derive(Clone,Copy,Debug)]
struct Rule {
    x: Page,
    y: Page,
}

type Page = u32;
type Book = Vec<Page>;

struct SafetyManual(Vec<Rule>, Vec<Book>);

trait CheckValidity {
    fn check_validity(self, book: Book) -> bool;
    fn correct(self, book: Book) -> Book;
}

impl CheckValidity for Rule {
    fn check_validity(self, book: Book) -> bool {
        let page_x = book.iter().position(|p| *p == self.x);
        let page_y = book.iter().position(|p| *p == self.y);
        match (page_x, page_y) {
            (Some(x), Some(y)) => x < y,
            _ => true
        }
    }

    fn correct(self, book: Book) -> Book {
        let page_x = book.iter().position(|p| *p == self.x);
        let page_y = book.iter().position(|p| *p == self.y);
        let mut book = book.clone();

        match (page_x, page_y) {
            (Some(x), Some(y)) => {
                if x > y {
                    book.swap(x,y);
                }
                book
            },
            _ => {
                book
            }
        }

    }
}

fn check_rules(input: &str, correct: bool) -> u32 {
    let prog = get_update_record(input);
    let rules = prog.0;
    let mut total: Page = 0;

    for book in prog.1 {
        let valid = rules.iter()
            .all(|rule| rule.check_validity(book.to_vec()));

        if valid && !correct {
            let mid_index = book.len() / 2;
            let mid_value = book[mid_index];
            total = total + mid_value;
        }
        else if correct {
            let mut new_entry = book.clone();
            let mut is_valid = false;

            while !is_valid {
                for rule in rules.iter() {
                    new_entry = rule.correct(new_entry.clone());
                }

                is_valid = rules.iter().all(|r| r.check_validity(new_entry.to_vec()));
            }
            let mid_index = new_entry.len() / 2;
            let mid_value = new_entry[mid_index];
            total = total + mid_value;
        }
    }

    return total.try_into().unwrap();
}

fn get_update_record(input: &str) -> SafetyManual {
    let lines = input.lines();
    let mut rules: Vec<Rule> = vec![];
    let mut books: Vec<Book> = vec![];

    for line in lines {
        let is_rule = line.contains("|");

        if is_rule {
            let nums: Vec<u32>  = line.split("|")
                .map(|x| {
                    match x.trim().parse::<Page>() {
                        Ok(value) => value,
                        Err(_) => 0
                    }
                })
                .collect();

            if nums.len() == 2 {
                rules.push(Rule{
                    x: nums[0],
                    y: nums[1]
                });
            }
        }
        else {
            let nums: Vec<Page> = line.split(",")
                    .map(|x| {
                        match x.trim().parse::<Page>() {
                            Ok(value) => value,
                            Err(_) => 0
                        }
                    })
                    .collect();

            if nums.len() > 1 {
                books.push(nums);
            }
        }
    }

    SafetyManual(rules, books)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "#;

        assert_eq!(check_rules(input, false), 143);
    }

    #[test]
    fn test_init1() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
        "#;

        assert_eq!(check_rules(input, false), 61);
    }

    #[test]
    fn test_init2() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
        "#;

        assert_eq!(check_rules(input, false), 143);
    }

    #[test]
    fn test_p2() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "#;

        assert_eq!(check_rules(input, true), 123);
    }

    #[test]
    fn test_p2_single() {
        let input = r#"
            29|13

            61,13,29
        "#;

        assert_eq!(check_rules(input, true), 29);
    }

    #[test]
    fn test_p2_single2() {
        let input = r#"
            97|75
            47|53

            75,97,47,61,53
        "#;

        assert_eq!(check_rules(input, true), 47);
    }

    #[test]
    fn test_p2_single3() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            97,13,75,29,47
        "#;

        assert_eq!(check_rules(input, true), 47);
    }
}