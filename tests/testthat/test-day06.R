test_that("day 6", {
  # part 1
  expect_equal(f06a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
  expect_equal(f06a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5)
  expect_equal(f06a("nppdvjthqldpwncqszvftbrmjlhg"), 6)
  expect_equal(f06a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10)
  expect_equal(f06a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11)

  # part 2
  expect_equal(f06b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19)
  expect_equal(f06b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23)
  expect_equal(f06b("nppdvjthqldpwncqszvftbrmjlhg"), 23)
  expect_equal(f06b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29)
  expect_equal(f06b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26)

})
