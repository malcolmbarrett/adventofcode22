test_that("day 7 works", {
  x <- readLines(test_path("test_input07.txt"))
  expect_equal(f07a(x), 95437)
  expect_equal(f07b(x), 24933642)
})
