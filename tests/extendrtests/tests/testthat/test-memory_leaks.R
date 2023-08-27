


test_that("extendr conversions/panics does not leak", {

  ## get list of rust_functions and value generator functions to test.
  e_pkg = environment(leak_implicit_doubles)
  fnames = c(

    "leak_implicit_doubles",# single burn-in function

    "leak_implicit_doubles",
    "leak_implicit_strings",

    "leak_arg2_try_implicit_doubles",
    "leak_arg2_try_implicit_strings"
  )
  rust_f_list = mget(fnames,envir = e_pkg)
  names(rust_f_list)[1] = "burn_in_test" #rename as burn-in

  value_f_list = list(
    big_chrvec = \() replicate(1E1,paste(sample(letters,1E1, replace = TRUE),collapse = "")),
    big_altvec = \() 1:(1E2),
    big_dblvec = \() (1:(1E2)) - 1.1
  )

  ## global memory, to minimize allocations while measuring
  glb_i = 0
  glb_mem_before      = NA_real_
  glb_mem_before_10   = NA_real_
  glb_mem_after       = NA_real_
  glb_mem_after_10    = NA_real_
  glb_mem_after_gc    = NA_real_
  glb_mem_after_gc_10 = NA_real_
  glb_is_error = NA


  #a function to measure
  score_leak <- function(f_rust, f_value, verbose = FALSE, n_repeats = 2){

    glb_mem_before <<- lobstr::mem_used()

    #run rust function possibly with the wrong arg types, catch error
    out = (\() tryCatch({
      #two arg case
      if(length(formals(f_rust))>1 ) {
          f_rust(rnorm(1E2),f_value())
      #one arg case
      } else {
          f_rust(f_value())
        }
      }, error = \(err) "ERROR")
    )()

    #measure mem usage and garbage collect and measure again
    glb_mem_after <<- lobstr::mem_used()
    glb_is_error <<- isTRUE(out == "ERROR")
    rm(out)
    gc(verbose = FALSE)
    glb_mem_after_gc <<- lobstr::mem_used()


    # now run again n_repeats
    glb_mem_before_10 <<- lobstr::mem_used()
    for (i in 1:n_repeats) {
      out = (\() tryCatch({
        if(length(formals(f_rust))>1 ) {
          f_rust(rnorm(1E2),f_value())
        } else {
          f_rust(f_value())
        }
      }, error = \(err) "ERROR"))()
    }
    glb_mem_after_10 <<- lobstr::mem_used()

    # drop garbage collect measure
    rm(out,i)
    gc(verbose = FALSE)
    glb_mem_after_gc_10 <<- lobstr::mem_used()


    # collect results
    list(
      total_mem_before = glb_mem_before,
      is_error = glb_is_error,
      leak_size_1  = glb_mem_after_gc -glb_mem_before,
      leak_size_repeat = (glb_mem_after_gc_10 - glb_mem_before_10) / n_repeats
    )
  }


  for(rust_fun_name in  names(rust_f_list)) {
    for(value_fun_name in names(value_f_list)) {

      mem_result = score_leak(
        rust_f_list[[rust_fun_name]], value_f_list[[value_fun_name]]
      )
      #print(mem_result$total_mem_before)
      if(rust_fun_name != "burn_in_test") {

        expect_identical(
          list(
            rust_f = rust_fun_name,
            input_val = value_fun_name,
            leak = as.double(mem_result$leak_size_repeat)
          ),
          list(
            rust_f = rust_fun_name,
            input_val = value_fun_name,
            leak = 0
          )
        )
      }


    }
  }

 #This test verifies score_leak() can detect a ~40kb leak
  #isch 5000*8~40000~40kb is expected leaked
  expect_true(
    as.numeric(score_leak(leak_positive_control,\()rnorm(5000))$leak_size_repeat) > 10000
  )

  # This test verifies score_leak will always find less than 256 bytes for a negative control
  # Without burn_in there is an increment of 128 bytes, which is not
  # accounted for. However at about ~10 more runs this 128bytes per run are released at once.
  expect_true(
    as.numeric(score_leak(leak_negative_control,\()rnorm(5000))$leak_size_repeat) <= 256
  )
  
})

