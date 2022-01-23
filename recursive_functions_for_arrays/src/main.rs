fn main() {
    reverse("abcdefghi");
    println!();
    common_part("111258", "113789");
}

fn common_part(a: &str, b: &str) {
    if a.chars().count() == 0 || b.chars().count() == 0 {
        return;
    }

    let head_a = head(a);
    let head_b = head(b);

    if head_a == head_b {
        print!("{head_a}");
        common_part(tail(a), tail(b))
    } else if head_a > head_b {
        common_part(a, tail(b))
    } else {
        common_part(tail(a), b)
    }
}

fn reverse(a: &str) {
    if a.chars().count() > 1 {
        reverse(tail(a));
    }
    print!("{}", head(a));
}

fn head(a: &str) -> char {
    a.chars().nth(0).unwrap()
}

fn tail(a: &str) -> &str {
    &a[1..]
}