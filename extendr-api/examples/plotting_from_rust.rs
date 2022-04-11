use extendr_api::{eval_string, eval_string_with_params, test, Result};
use extendr_api::{Doubles, R};

fn main() {
    test! {
        let x = Doubles::from_values((0..100).map(|i| i as f64 / 20.0));

        // let y = Doubles::from_values(x.iter().map(|x| x.inner().sin()));
        let y = Doubles::from_values((0..100).map(|i| (i as f64 / 20.0).sin()));

        // Set a PNG device
        R!(r#"png("/tmp/sin_plot.png")"#)?;

        // Plot x and y
        R!("plot({{&x}}, {{&y}})")?;

        // Linear model.
        R!("abline(lm({{y}} ~ {{x}}))")?;

        // Flush the device to the image.
        R!("dev.off()")?;
    }
}
