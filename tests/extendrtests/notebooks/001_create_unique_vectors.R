library(magrittr)
library(tidyverse)
devtools::load_all(path = "tests/extendrtests")

text_character_vector <-
  lorem::ipsum_words(1e6, collapse = FALSE)

to_unique_rstr(text_character_vector)
to_unique_str(text_character_vector)


bench::mark(
  # dplyr = dplyr::n_distinct(text_character_vector),
  rstr = to_unique_rstr(text_character_vector),
  str = to_unique_str(text_character_vector),
  check = TRUE
) -> bm_n_unique

bm_n_unique %>% 
  select(expression, is.atomic)
# # A tibble: 2 × 9
# expression      min   median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
# <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl> <int> <dbl>   <bch:tm>
# 1 rstr          539ms    539ms      1.86        0B        0     1     0      539ms
# 2 str           840ms    840ms      1.19        0B        0     1     0      840ms