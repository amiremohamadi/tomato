use anyhow::bail;
use serde::Deserialize;
use wirefilter::{Field, Filter, Scheme};

lazy_static::lazy_static! {
    pub static ref SCHEME: Scheme = Scheme !{
            ip.src: Ip,
    };
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct CompiledRule {
    pub filter: Filter<'static>,
    pub fields: Vec<Field<'static>>,
}

impl CompiledRule {
    pub fn new(expr: &str) -> Result<Self, anyhow::Error> {
        let ast = match SCHEME.parse(expr) {
            Ok(x) => x,
            Err(e) => bail!("{:?}", e),
        };

        let mut fields = vec![];
        SCHEME
            .iter()
            .filter(|(x, _)| ast.uses(x).unwrap_or(false))
            .filter_map(|(x, _)| SCHEME.get_field(x).ok())
            .for_each(|f| {
                if !fields.contains(&f) {
                    fields.push(f);
                }
            });
        let filter = ast.compile();

        Ok(Self { filter, fields })
    }
}

impl TryFrom<String> for CompiledRule {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}
