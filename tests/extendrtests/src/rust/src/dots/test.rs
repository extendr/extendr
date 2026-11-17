use extendr_api::dots::Dots;
use extendr_api::prelude::*;
use extendr_api::Result;

#[derive(Debug, Clone)]
pub struct BulletList(Strings);

impl TryFrom<&Robj> for BulletList {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        if !robj.inherits("bullet_list") {
            return Err(Error::TypeMismatch(robj.clone()));
        }
        Ok(BulletList(Strings::try_from(robj)?))
    }
}

impl TryFrom<Robj> for BulletList {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        Self::try_from(&robj)
    }
}

impl From<BulletList> for Robj {
    fn from(x: BulletList) -> Self {
        x.0.into()
    }
}

/// @export
#[extendr(dots)]
fn bullet_list() -> Result<BulletList> {
    let items = match Dots::find() {
        Some(dots) => dots
            .into_iter()
            .map(|dot| match dot.value()? {
                Some(value) => Rstr::try_from(value),
                None => Ok(Rstr::na()),
            })
            .collect::<Result<Vec<Rstr>>>()?,
        None => Vec::new(),
    };
    let mut items = Strings::from_values(items);
    items.set_class(["bullet_list"])?;
    Ok(BulletList(items))
}

/// @export
#[extendr(dots, invisible, r_name = "print.bullet_list")]
fn print_bullet_list(x: BulletList) -> Result<BulletList> {
    let dots = Dots::find();
    let arg = |name: &str| match dots.as_ref().and_then(|d| d.get(name)) {
        Some(dot) => dot.value(),
        None => Ok(None),
    };
    let width = arg("width")?.map(|w| usize::try_from(&w)).transpose()?;
    let symbol = arg("symbol")?
        .map(String::try_from)
        .transpose()?
        .unwrap_or_else(|| "•".to_string());

    for item in x.0.iter() {
        let text: String = if item.is_na() {
            "NA".to_string()
        } else {
            match width {
                Some(width) => item.as_ref().chars().take(width).collect(),
                None => item.as_ref().to_string(),
            }
        };
        rprintln!("{symbol} {text}");
    }
    Ok(x)
}

extendr_module! {
    mod test;
    fn bullet_list;
    fn print_bullet_list;
}
