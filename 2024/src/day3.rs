use regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let muls = extract_muls_from_string(input);
    multiply_pairs(muls).try_into().unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let enabled_areas = extract_enabled_areas(input);
    let muls = enabled_areas
        .iter()
        .map(|area| extract_muls_from_string(area))
        .flatten()
        .collect::<Vec<(i32, i32)>>();
    multiply_pairs(muls).try_into().unwrap()
}

// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
// need to extract the mul commands and remove the rest
// planning to use a regex to extract the mul commands
fn extract_muls_from_string(string: &str) -> Vec<(i32, i32)> {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut muls = Vec::new();
    for cap in re.captures_iter(string) {
        let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        muls.push((x, y));
    }
    muls
}

fn multiply_pairs(pairs: Vec<(i32, i32)>) -> i32 {
    pairs.iter().map(|(x, y)| x * y).sum()
}

// xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
// the do() and don't() commands enable a disable the mul command. It Starts enabled
//  for the following function i am going to split the strings up and extract the pices of string that are inbetween the do and dont commands
fn extract_enabled_areas(string: &str) -> Vec<&str> {
    let mut enabled_areas = Vec::new();

    // split the string by the don't command
    let dont_split: Vec<&str> = string.split("don't()").collect();

    // first one will always be enabled
    enabled_areas.push(dont_split[0]);

    // loop through the rest of the strings try to find the do() command and add the right side to the enabled_areas
    for i in 1..dont_split.len() {
        let do_split: Vec<&str> = dont_split[i].split("do()").collect();
        // if there is no do command then do nothing
        if do_split.len() < 2 {
            continue;
        }
        for j in 1..do_split.len() {
            enabled_areas.push(do_split[j]);
        }
    }
    enabled_areas
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_muls_from_string() {
        let string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let muls = extract_muls_from_string(string);
        assert_eq!(muls, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_multiply_pairs() {
        let pairs = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(multiply_pairs(pairs), 161);
    }

    #[test]
    // xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    fn test_extract_enabled_areas() {
        let string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let areas = extract_enabled_areas(string);
        assert_eq!(areas, vec!["xmul(2,4)&mul[3,7]!^", "?mul(8,5))"]);

        let muls = areas
            .iter()
            .map(|area| extract_muls_from_string(area))
            .flatten()
            .collect::<Vec<(i32, i32)>>();

        assert_eq!(muls, vec![(2, 4), (8, 5)]);

        let result = multiply_pairs(muls);
        assert_eq!(result, 48);
    }

    // -:-]what()(+/mul(957,396)?mul(550,844)%+why())-? #}from()mul(488,628)%} ~**mul(770,931)$~mul(791,733)<{mul(985,350)<#why()don't()what()select()$what())]what()who()mul(327,185))<^^mul(542,68)#?who()<from()';^how()mul(619,952)/where(){(!);'@,mul(551,161)select()>when()do()from()mul(51,291)[where()!{]/}'@?mul(233,511)@what()]mul(311,967))&who()how()mul(839,578)^who()]}mul(266,735){mul(176,670)mul(154,710)*select()](':^,mul(531,801)# *why()why()mul(30,325)~,where();select()select()}-/when()mul(512,729)+where();[mul(720,339)[~*when()mul(722,867)!);{+mul(582,286)^:)what()@mul(604,485) (who()why()who()from()
    #[test]
    fn test_extracted_again() {
        let string = "-:-]what()(+/mul(957,396)?mul(550,844)%+why())-? #}from()mul(488,628)%} ~**mul(770,931)$~mul(791,733)<{mul(985,350)<#why()don't()what()select()$what())]what()who()mul(327,185))<^^mul(542,68)#?who()<from()';^how()mul(619,952)/where(){(!);'@,mul(551,161)select()>when()do()from()mul(51,291)[where()!{]/}'@?mul(233,511)@what()]mul(311,967))&who()how()mul(839,578)^who()]}mul(266,735){mul(176,670)mul(154,710)*select()](':^,mul(531,801)# *why()why()mul(30,325)~,where();select()select()}-/when()mul(512,729)+where();[mul(720,339)[~*when()mul(722,867)!);{+mul(582,286)^:)what()@mul(604,485) (who()why()who()from()";
        let areas = extract_enabled_areas(string);
        dbg!(&areas);

        assert_eq!(areas, vec!["-:-]what()(+/mul(957,396)?mul(550,844)%+why())-? #}from()mul(488,628)%} ~**mul(770,931)$~mul(791,733)<{mul(985,350)<#why()", "from()mul(51,291)[where()!{]/}'@?mul(233,511)@what()]mul(311,967))&who()how()mul(839,578)^who()]}mul(266,735){mul(176,670)mul(154,710)*select()](':^,mul(531,801)# *why()why()mul(30,325)~,where();select()select()}-/when()mul(512,729)+where();[mul(720,339)[~*when()mul(722,867)!);{+mul(582,286)^:)what()@mul(604,485) (who()why()who()from()"]);

        let muls = areas
            .iter()
            .map(|area| extract_muls_from_string(area))
            .flatten()
            .collect::<Vec<(i32, i32)>>();

        dbg!(muls);
    }
}

// should be 161
