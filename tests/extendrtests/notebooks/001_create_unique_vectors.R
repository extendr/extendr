devtools::load_all()

text_character_vector <-
  lorem::ipsum_words(1e5, collapse = FALSE)

to_unique_rstr(text_character_vector)
to_unique_str(text_character_vector)


bench::mark(
  dplyr = dplyr::n_distinct(text_character_vector),
  rstr = to_unique_rstr(text_character_vector),
  str = to_unique_str(text_character_vector),
  check = TRUE
)
