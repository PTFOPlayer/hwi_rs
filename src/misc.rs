pub fn prec_100(mut f: f64) -> f64 {
    f *= 100.;
    f = f.round();
    f / 100.
}
