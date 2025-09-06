pub fn runlength_encoding<T>(v: &Vec<T>) -> Vec<(T, usize)>
where
    T: Copy + PartialEq,
{
    if v.is_empty() {
        return Vec::<(T, usize)>::new();
    }
    let mut res = vec![(v[0], 1_usize)];
    for e in v {
        if res.last().unwrap().0 == *e {
            res.last_mut().unwrap().1 += 1;
        } else {
            res.push((*e, 1));
        }
    }
    res
}
