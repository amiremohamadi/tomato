use wirefilter::{Field, Filter, Scheme};
use anyhow::bail;

lazy_static::lazy_static! {
    pub static ref SCHEME: Scheme = Scheme !{
            ip.src: Ip,
    };
}

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

        Ok(Self {
            filter,
            fields,
        })
    }
}
