test_that("day 4 works", {
  expect_equal(f05a(test_path("test_input05.txt"), cutoff = 3, columns = 3), "CMZ")
  expect_equal(f05b(test_path("test_input05.txt"), cutoff = 3, columns = 3), "MCD")
})
