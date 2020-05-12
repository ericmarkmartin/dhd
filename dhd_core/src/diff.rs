use difference::Difference;

#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum DiffType {
    Same,
    Add,
    Rem,
}

#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct Diff {
    diff_type: DiffType,
    data: String,
}

impl From<Difference> for Diff {
    fn from(difference: Difference) -> Self {
        use Difference::*;
        match difference {
            Same(data) => Diff {
                diff_type: DiffType::Same,
                data,
            },
            Add(data) => Diff {
                diff_type: DiffType::Add,
                data,
            },
            Rem(data) => Diff {
                diff_type: DiffType::Rem,
                data,
            },
        }
    }
}

impl From<Diff> for Difference {
    fn from(diff: Diff) -> Self {
        use DiffType::*;
        let data = diff.data;
        match diff.diff_type {
            Same => Difference::Same(data),
            Add => Difference::Add(data),
            Rem => Difference::Rem(data),
        }
    }
}
