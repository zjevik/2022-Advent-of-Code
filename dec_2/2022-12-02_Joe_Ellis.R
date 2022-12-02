
# ADVENT OF CODE - DAY 2

library(readr)
library(purrr)
library(magrittr)
library(tibble)
library(dplyr)

input <- readr::read_csv(
  file = "input.csv",
  skip_empty_rows = FALSE
)

# PART 1 - strategy guide guess

# build a tibble to represent the outcomes based on our guess as to how the
# strategy guide works
games_guess <- tibble::tibble(
  "opponent" = c("A", "A", "A", "B", "B", "B", "C", "C", "C"),
  "you" = rep(x = c("X", "Y", "Z"), times = 3),
  "outcome" = c(4, 8, 3, 1, 5, 9, 7, 2, 6)
)

# join the game outcomes to the games input
results_guess <- input %>%
  dplyr::left_join(games_guess)

# get the total score for all the games
score_guess <- results_guess %>%
  dplyr::pull(outcome) %>%
  sum()


# PART 2 - strategy guide known

# build a tibble to represent the outcomes based on how the strategy guide
# actually works
games_known <- tibble::tibble(
  "opponent" = c("A", "A", "A", "B", "B", "B", "C", "C", "C"),
  "you" = rep(x = c("X", "Y", "Z"), times = 3),
  "outcome" = c(3, 4, 8, 1, 5, 9, 2, 6, 7)
)

# join the game outcomes to the games input
results_known <- input %>%
  dplyr::left_join(games_known)

# get the total score for all the games
score_known <- results_known %>%
  dplyr::pull(outcome) %>%
  sum()
