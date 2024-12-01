fn split_lists(lists: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];

    let rows = lists.trim().split("\n");

    for row in rows {
        let values: Vec<&str> = row.split("   ").collect();

        if values.len() != 2 {
            println!("failing values: {:?}", values);
            panic!("row does not contain two values");
        }

        let v1 = values[0];
        let v1 = v1.parse::<usize>().unwrap();

        let v2 = values[1];
        let v2 = v2.parse::<usize>().unwrap();

        list1.push(v1);
        list2.push(v2);
    }

    return (list1, list2);
}

fn distance(val1: &usize, val2: &usize) -> usize {
    if val1 >= val2 {
        return val1 - val2;
    }

    return val2 - val1;
}

pub fn total_distance(lists: &str) -> usize {
    let (list1, list2) = split_lists(&lists);

    let mut list1 = list1.to_vec();
    let mut list2 = list2.to_vec();

    list1.sort();
    list2.sort();

    let mut total_d = 0;

    for (index, val1) in list1.iter().enumerate() {
        let val2 = list2[index];
        total_d += distance(&val1, &val2);
    }

    return total_d;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_distance() {
        let (v1, v2) = (3, 7);
        let result = super::distance(&v1, &v2);
        assert_eq!(result, 4);

        let (v1, v2) = (7, 3);
        let result = super::distance(&v1, &v2);
        assert_eq!(result, 4);

        let (v1, v2) = (1, 1);
        let result = super::distance(&v1, &v2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_split_list() {
        let example = "3   4
4   3
2   5
";
        let (list1, list2) = super::split_lists(&example);
        assert_eq!(list1, vec![3, 4, 2]);
        assert_eq!(list2, vec![4, 3, 5]);
    }

    #[test]
    fn test_total_distance() {
        let example = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = super::total_distance(example);
        assert_eq!(result, 11);
    }
}
