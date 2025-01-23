use pretty::{DocAllocator, DocBuilder};

use super::{stmt::Stmt, Logic};

impl Logic {
    pub fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Logic::OneOf(vec) => allocator
                .text("(")
                .append(
                    allocator
                        .intersperse(
                            vec.iter().map(|l| l.pretty(allocator)),
                            allocator.text(" | "),
                        )
                        .group()
                        .nest(2),
                )
                .append(allocator.text(")")),
            Logic::AllOf(vec) => allocator
                .text("(")
                .append(
                    allocator
                        .intersperse(
                            vec.iter().map(|l| l.pretty(allocator)),
                            allocator.text(" & "),
                        )
                        .group()
                        .nest(2),
                )
                .append(allocator.text(")")),
            Logic::Stmt(stmt) => stmt.pretty(allocator),
            Logic::True => allocator.text("true"),
            Logic::False => allocator.text("false"),
        }
    }
}

impl Stmt {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Stmt::Exactly { ty, is } => allocator
                .text(ty.to_string())
                .append(allocator.text(" = "))
                .append(is.to_string()),
            Stmt::Extends { sub, super_ } => allocator
                .text(sub.to_string())
                .append(allocator.text(": "))
                .append(super_.to_string()),
            Stmt::HasMember { .. } => todo!(),
        }
    }
}
