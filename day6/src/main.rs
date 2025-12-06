use std::fs;
use eval::eval;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: u64
}

trait InputGetter {
    fn get_input(&self) -> String;
    
}

struct LocalFileInputGetter {
    path: &'static str,
}

impl InputGetter for LocalFileInputGetter {
    fn get_input(&self) -> String {
        return fs::read_to_string(self.path).expect("Input file is expected");
    }
}

fn parse_data(contents: &String) -> Vec<Vec<&str>> {
    let line_data: Vec<Vec<&str>> = contents.lines().map(|line| line.split(" ").filter(|x| *x != "").collect()).collect();
    return line_data
}

fn part1(contents: &String) -> Option<Answer> {
    let mut answer = 0.0;

    let data = parse_data(contents);

    let first_line = data.get(0).expect("Expected a line here");

    let mut expressions: Vec<String> = vec!["".to_string(); first_line.len()];

    for (i, _) in first_line.iter().enumerate() {
        for line in &data[..data.len()-1] {
            expressions[i] += line[i];
            expressions[i] += data[data.len()-1][i];
        }
    }

    dbg!(&expressions);

    for expression in expressions {
        let string_expression = expression.to_string();
        if &string_expression == "" {
            continue;
        }
        dbg!(&string_expression);
        let eval_expression = &string_expression[..string_expression.len()-1];
        dbg!(eval_expression);
        let out = eval(eval_expression).expect("Expected a value here").as_f64().expect("Expected a float here");
        dbg!(out);
        answer += out;
    }

    dbg!(answer);

    let answer_64 = answer.round() as u64;

    return Some(Answer { answer: answer_64 })

}

// Part 1 attempted answers
// 566612075051 too low

fn part2(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");
    let answer = 0;

    return Some(Answer { answer })
}

// Part 2 attempted answers

fn main() {

    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let doing_part1 = true;
    if doing_part1 {
        let result1 = part1(&contents);
        println!("Part1 result {result1:?}");
    } else {
        let result2 = part2(&contents);
        println!("Part2 result {result2:?}");
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        contents: String,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                contents: "123 328  51 64\n\
                            45 64  387 23\n\
                             6 98  215 314\n\
                           *   +   *   +  "
                .to_string(), 
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents);

        dbg!(eval("123*45*6*").expect("Expected value").as_f64().expect("Expected float"));
        assert_eq!(result, Some(Answer { answer: 4277556 }));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 6503327062445 }));
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 43 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}