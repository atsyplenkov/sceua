test_that("sceua converges on a two-dimensional sphere", {
  result <- sceua(
    fn = function(x) sum(x^2),
    lower = c(-5, -5),
    upper = c(5, 5),
    max_evaluations = 5000L,
    kstop = 5L,
    pcento = 1e-8,
    seed = 1969L,
    complexes = 5L
  )

  expect_s3_class(result, "sceua")
  expect_length(result$par, 2)
  expect_lt(result$value, 1e-6)
  expect_gt(result$counts, 0)
  expect_gt(result$iterations, 0)
  expect_in(
    result$termination,
    c("objective_convergence", "parameter_convergence", "max_evaluations")
  )
  expect_true(is.data.frame(result$history))
})

test_that("sceua passes extra arguments to the objective", {
  fn <- function(x, target) sum((x - target)^2)

  result <- sceua(
    fn = fn,
    lower = c(-5, -5),
    upper = c(5, 5),
    target = c(1, 2),
    max_evaluations = 5000L,
    seed = 1969L
  )

  expect_lt(sum((result$par - c(1, 2))^2), 1e-2)
})

test_that("sceua validates bound lengths", {
  expect_error(
    sceua(fn = function(x) sum(x^2), lower = c(-5), upper = c(5, 5)),
    "same length"
  )
})

test_that("sceua respects initial point", {
  result <- sceua(
    fn = function(x) sum(x^2),
    lower = c(-5, -5),
    upper = c(5, 5),
    initial = c(1, 1),
    max_evaluations = 100L,
    seed = 1969L
  )

  expect_length(result$par, 2)
  expect_true(result$value < Inf)
})
