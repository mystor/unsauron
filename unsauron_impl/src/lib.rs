extern crate proc_macro;
extern crate syn;
extern crate synom;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate proc_macro_hack;

use syn::*;
use synom::Synom;
use syn::fold::{Folder, noop_fold_expr};
use std::default::Default;

fn sauron(e: Expr) -> Box<Expr> {
    Box::new(Expr {
        node: ExprKind::AddrOf(ExprAddrOf {
            expr: Box::new(Expr {
                node: ExprKind::Paren(ExprParen {
                    expr: Box::new(e),
                    paren_token: Default::default(),
                }),
                attrs: vec![],
            }),
            mutbl: Mutability::Immutable,
            and_token: Default::default(),
        }),
        attrs: vec![],
    })
}

struct F;
impl Folder for F {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        Expr {
            node: match e.node {
                ExprKind::Binary(b) => {
                    ExprKind::Binary(ExprBinary {
                        left: sauron(self.fold_expr(*b.left)),
                        right: sauron(self.fold_expr(*b.right)),
                        ..b
                    })
                },
                _ => return noop_fold_expr(self, e),
            },
            attrs: e.attrs
        }
    }
}

proc_macro_expr_impl! {
    pub fn expand_unsauron(input: &str) -> String {
        let e = Expr::parse_str_all(input)
            .expect("Input should be a valid expression");
        let e = F.fold_expr(e);
        quote!(#e).to_string()
    }
}
