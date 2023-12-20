pub fn prec(mut f: f64) -> f64 {
    f *= 1000.;
    f = f.round();
    f / 1000.
}
