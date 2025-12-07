use eval::eval;
use std::fs;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: u64,
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

fn parse_data_part1(contents: &String) -> Vec<Vec<&str>> {
    let line_data: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split(" ").filter(|x| *x != "").collect())
        .collect();
    return line_data;
}

fn do_expressions(expressions: Vec<String>) -> u64 {
    let mut answer = 0.0;

    dbg!(&expressions);

    for expression in expressions {
        let string_expression = expression.to_string();
        if &string_expression == "" {
            continue;
        }
        dbg!(&string_expression);
        let eval_expression = &string_expression[..string_expression.len() - 1];
        dbg!(eval_expression);
        let out = eval(eval_expression)
            .expect("Expected a value here")
            .as_f64()
            .expect("Expected a float here");
        dbg!(out);
        answer += out;
    }

    dbg!(answer);

    let answer_64 = answer.round() as u64;

    return answer_64;
}

fn part1(contents: &String) -> Option<Answer> {
    let data = parse_data_part1(contents);
    let first_line = data.get(0).expect("Expected a line here");

    let mut expressions: Vec<String> = vec!["".to_string(); first_line.len()];

    for (i, _) in first_line.iter().enumerate() {
        for line in &data[..data.len() - 1] {
            expressions[i] += line[i];
            expressions[i] += data[data.len() - 1][i];
        }
    }

    dbg!(&expressions);

    return Some(Answer {
        answer: do_expressions(expressions),
    });
}

// Part 1 attempted answers
// 566612075051 too low

fn parse_data_part2(contents: &String) -> (Vec<Vec<String>>, Vec<String>) {
    // Part 1 parses the data into each
    // part1_data = [["123","328","51","64",]
    //              ,["45","64","387","23",]
    //              ,["6","98","215","314",]
    //              ,["*","+","*","+",],]

    println!("{contents}");
    let string_lines = contents
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let first_line = string_lines.get(0).expect("Expected the first line here");

    let mut values: Vec<String> = vec![];

    for i in (0..first_line.len()).rev() {

        let mut current_value = "".to_string();

        for line in &string_lines[..string_lines.len() - 1] {
            current_value += &line
                .chars()
                .nth(i)
                .expect("Expected a character here")
                .to_string();
        }
        values.push(current_value);
    }

    let mut out_data: Vec<Vec<String>> = vec![];

    let mut current_line: Vec<String> = vec![];

    dbg!(&values);

    for value in values {
        if value.chars().filter(|c| *c == ' ').count() == value.len()  {
            out_data.push(current_line);
            current_line = vec![];
        } else {
            current_line.push(value.replace(" ", ""));
        }
    }
    out_data.push(current_line);

    // Now we just need to get the signs!

    let signs: Vec<String> = string_lines
        .get(string_lines.len() - 1)
        .expect("Expected the last line here")
        .split(" ")
        .filter(|x| *x != "")
        .map(|x| x.to_string())
        .collect::<Vec<String>>().into_iter().rev().collect();

    dbg!(&out_data);

    // Needs to
    return (out_data, signs);
}

fn part2(contents: &String) -> Option<Answer> {
    let (data, signs)= parse_data_part2(contents);


    // Expresions is a Vector where each item is a String that is a expression which is ready to parse
    let mut expressions: Vec<String> = vec![];


    for (i, line) in data.iter().enumerate() {
        let mut expression = "".to_string();
        for value in line {
            expression.push_str(&value);
            expression.push_str(signs.get(i).expect("Expected a sign from this list"));
        }
        expressions.push(expression);
    }

    dbg!(&expressions);

    return Some(Answer {
        answer: do_expressions(expressions),
    });
}

// Part 2 attempted answers

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let doing_part1 = false;
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
                contents: "123 328  51 64 
 45 64  387 23 
  6 98  215 314
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

        dbg!(
            eval("123*45*6*")
                .expect("Expected value")
                .as_f64()
                .expect("Expected float")
        );
        assert_eq!(result, Some(Answer { answer: 4277556 }));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(
            result,
            Some(Answer {
                answer: 6503327062445
            })
        );
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 3263827 }));
        // assert_eq!(result, Some(Answer { answer: 3263828 }));
    }

    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 9640641878593 }));
    }
}
