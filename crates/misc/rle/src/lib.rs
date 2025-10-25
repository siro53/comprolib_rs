pub fn runlength_encoding<T>(v: &Vec<T>) -> Vec<(T, usize)>
where
    T: Copy + PartialEq,
{
    if v.is_empty() {
        return Vec::<(T, usize)>::new();
    }
    let mut res = vec![(v[0], 1_usize)];
    for i in 1..v.len() {
        if res.last().unwrap().0 == v[i] {
            res.last_mut().unwrap().1 += 1;
        } else {
            res.push((v[i], 1));
        }
    }
    res
}
