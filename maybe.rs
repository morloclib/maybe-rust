pub fn morloc_maybe_cat_maybes<A: Clone>(xs: &Vec<Option<A>>) -> Vec<A> {
    xs.iter().flatten().cloned().collect()
}
