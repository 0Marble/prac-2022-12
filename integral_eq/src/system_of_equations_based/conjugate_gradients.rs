/*
void discrepency(const double* mat, const double* x, const double* f, double* r,
                 uint32_t n) {
  memset(r, 0, n * sizeof(double));
  for (uint32_t i = 0; i < n; i++) {
    for (uint32_t j = 0; j < n; j++) {
      r[i] += mat[i * n + j] * x[j];
    }
    r[i] -= f[i];
  }
}
*/

fn discrepency(mat: &[f64], x: &[f64], f: &[f64], r: &mut [f64], n: usize) {
    for i in 0..n {
        r[i] = -f[i];
        for j in 0..n {
            r[i] += mat[i * n + j] * x[j];
        }
    }
}

/*
void apply(const double* mat, const double* x, double* y, uint32_t n) {
  memset(y, 0, n * sizeof(double));
  for (uint32_t i = 0; i < n; i++) {
    for (uint32_t j = 0; j < n; j++) {
      y[i] += mat[i * n + j] * x[j];
    }
  }
}
*/

pub fn apply(mat: &[f64], x: &[f64], y: &mut [f64], n: usize) {
    for i in 0..n {
        y[i] = 0.0;
        for j in 0..n {
            y[i] += mat[i * n + j] * x[j];
        }
    }
}

/*
void mult_mat(const double* a, const double* b, double* c, uint32_t n) {
  for (uint32_t i = 0; i < n; i++) {
    for (uint32_t j = 0; j < n; j++) {
      c[i * n + j] = 0.0;
      for (uint32_t k = 0; k < n; k++)
        c[i * n + j] += a[i * n + k] * b[k * n + j];
    }
  }
}
*/

pub fn mult_mat(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    for i in 0..n {
        for j in 0..n {
            c[i * n + j] = 0.0;
            for k in 0..n {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
}

/*
double dot(const double* a, const double* b, uint32_t n) {
  double sum = 0.0;
  for (uint32_t i = 0; i < n; i++) {
    sum += a[i] * b[i];
  }
  return sum;
}
*/

fn dot(a: &[f64], b: &[f64], _: usize) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/*
MethodReturnType conjugate_gradient_method(const double* a, const double* b,
                                           const double* inv_b, double* x,
                                           const double* f, uint32_t n,
                                           double eps,
                                           uint32_t max_step_count) {
  std::vector<double> rk(n, 0.0), wk(n, 0.0), awk(n, 0.0);
  std::vector<double> prev_x(n, 0.0);
  uint32_t step_count = 0;

  for (uint32_t i = 0; i < n; i++) prev_x[i] = x[i];
  discrepency(a, prev_x.data(), f, rk.data(), n);
  double e = dot(rk.data(), rk.data(), n);
  if (std::isnan(e)) {
    return MethodReturnType{.had_error = true};
  } else if (e < eps * eps || step_count == max_step_count) {
    return MethodReturnType{.rk = std::sqrt(e), .step_count = step_count};
  }
  step_count += 1;

  apply(inv_b, rk.data(), wk.data(), n);
  apply(a, wk.data(), awk.data(), n);
  double wkrk = dot(wk.data(), rk.data(), n);
  double tau = wkrk / dot(awk.data(), wk.data(), n);

  for (uint32_t i = 0; i < n; i++) x[i] = prev_x[i] - tau * wk[i];

  double prev_tau = tau, prev_alpha = 1.0, prev_wkrk = wkrk;

  while (true) {
    discrepency(a, x, f, rk.data(), n);  // r[k] = Ax[k] - f
    double e = dot(rk.data(), rk.data(), n);
    if (std::isnan(e)) {
      return MethodReturnType{.had_error = true};
    } else if (e < eps * eps || step_count == max_step_count) {
      return MethodReturnType{.rk = std::sqrt(e), .step_count = step_count};
    }
    step_count += 1;

    apply(inv_b, rk.data(), wk.data(), n);  // w[k]=inv(B)r[k]
    apply(a, wk.data(), awk.data(), n);     // Aw[k]

    double wkrk = dot(wk.data(), rk.data(), n);
    double tau = wkrk / dot(awk.data(), wk.data(),
                            n);  // t[k+1] = <w[k],r[k]>/<Aw[k],w[k]>
    double alpha =
        1.0 / (1.0 - (tau * wkrk) /
                         (prev_tau * prev_alpha *
                          prev_wkrk));  // a[k+1] = 1/(1 - t[k+1]/t[k] * 1/a[k]
                                        // *<w[k],r[k]>/<w[k-1],r[k-1]>)

    for (uint32_t i = 0; i < n; i++) {
      double temp = x[i];
      x[i] = alpha * x[i] + (1.0 - alpha) * prev_x[i] - tau * alpha * wk[i];
      prev_x[i] = temp;
    }  // x[k+1] = a[k+1]x[k] + (1-a[k+1])x[k-1] - t[k+1]a[k+1]w[k]

    prev_alpha = alpha;
    prev_tau = tau;
    prev_wkrk = wkrk;
  }
}
*/

pub fn conjugate_gradient_method(
    a: &[f64],
    inv_b: &[f64],
    x: &mut [f64],
    f: &[f64],
    n: usize,
    eps: f64,
    max_iter_count: usize,
) {
    let mut rk = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    let mut wk = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    let mut awk = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    let mut prev_x = x.to_owned();

    discrepency(a, &prev_x, f, &mut rk, n);
    let e = dot(&rk, &rk, n);
    if e < eps * eps {
        return;
    }

    apply(inv_b, &rk, &mut wk, n);
    apply(a, &wk, &mut awk, n);
    let wkrk = dot(&wk, &rk, n);
    let tau = wkrk / dot(&awk, &wk, n);

    for i in 0..n {
        x[i] = prev_x[i] - tau * wk[i];
    }

    let mut prev_tau = tau;
    let mut prev_alpha = 1.0;
    let mut prev_wkrk = wkrk;

    for _ in 0..max_iter_count {
        discrepency(a, x, f, &mut rk, n);
        let e = dot(&rk, &rk, n);
        if e < eps * eps {
            return;
        }

        apply(inv_b, &rk, &mut wk, n);
        apply(a, &wk, &mut awk, n);

        let wkrk = dot(&wk, &rk, n);
        let tau = wkrk / dot(&awk, &wk, n);
        let alpha = 1.0 / (1.0 - (tau * wkrk) / (prev_tau * prev_alpha * prev_wkrk));

        for i in 0..n {
            let temp = x[i];
            x[i] = alpha * x[i] + (1.0 - alpha) * prev_x[i] - tau * alpha * wk[i];
            prev_x[i] = temp;
        }
        prev_alpha = alpha;
        prev_tau = tau;
        prev_wkrk = wkrk;
    }
}
