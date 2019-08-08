use proc_macro2;
use proc_macro2::Span;
use syn;

use heck::{CamelCase, SnakeCase};

use crate::{diagnostic_shim::*, meta::*};

struct Model {
    pub name: syn::Ident,
    pub id_type: Option<syn::Ident>,
    pub record_type: Option<syn::Ident>,
    pub table_name: Option<syn::Ident>,
}

impl Model {
    pub fn from_item(item: &syn::DeriveInput) -> Result<Self, Diagnostic> {
        let record_type = MetaItem::with_name(&item.attrs, "record_type").map(|m| m.expect_ident_value());
        let id_type = MetaItem::with_name(&item.attrs, "id_type").map(|m| m.expect_ident_value());
        let table_name = MetaItem::with_name(&item.attrs, "table_name").map(|m| m.expect_ident_value());
        Ok(Self {
            name: item.ident.clone(),
            id_type,
            record_type,
            table_name,
        })
    }
}

type Ident2 = proc_macro2::Ident;
type Literal2 = proc_macro2::Literal;

pub fn derive(item: syn::DeriveInput) -> Result<proc_macro2::TokenStream, Diagnostic> {
    let model = Model::from_item(&item)?;

    let struct_name = &model.name;
    let id_type = &model.id_type.unwrap_or(Ident2::new("ID", Span::call_site()));
    let record_type = &model.record_type.unwrap_or({
        let name = struct_name.to_string();
        let name = if name.ends_with("Dao") {
            name[0..name.len() - 3].to_camel_case()
        } else {
            name.to_camel_case()
        };
        Ident2::new(&name, Span::call_site())
    });
    let table_name = &model.table_name.unwrap_or({
        let name = struct_name.to_string();
        let name = if name.ends_with("Dao") {
            name[0..name.len() - 3].to_snake_case()
        } else {
            name.to_snake_case()
        };
        Ident2::new(&format!("{}s", name), Span::call_site())
    });

    let get_by_id_doc = Literal2::string(&format!(" Get {} by ID", record_type));
    let get_list_doc = Literal2::string(&format!(" Get list of {}", record_type));
    let get_count_doc = Literal2::string(&format!(" Get count of {}", record_type));
    let delete_doc = Literal2::string(&format!(" Delete {} by ID", record_type));
    let get_records = Ident2::new(
        &format!("get_{}s", record_type).to_snake_case(),
        Span::call_site(),
    );

    Ok(quote! {
      impl<'a> #struct_name<'a> {

            #[doc(hidden)]
            pub fn new(db: &'a PgConnection) -> Self {
                Self { db }
            }

            #[doc=#get_by_id_doc]
            pub fn get_by_id(&self, id: #id_type) -> Result<#record_type> {
                use crate::schema::#table_name::dsl::#table_name;

                assert!(id != 0, "id is zero");

                #table_name.find(id).first(self.db).map_err(From::from)
            }

            #[doc=#get_list_doc]
            pub fn #get_records(&self, offset: i64, limit: i64) -> Result<Vec<#record_type>> {
                use crate::schema::#table_name::dsl;

                assert!(offset > -1, "Invalid offset");
                assert!(limit > -1, "Invalid limit");
                assert!(limit < 1_000_000, "Invalid limit");

                dsl::#table_name
                    .filter(dsl::id.ne(0))
                    .offset(offset)
                    .limit(limit)
                    .load(self.db)
                    .map_err(From::from)
            }

            #[doc=#get_count_doc]
            pub fn count(&self) -> Result<i64> {
                use crate::schema::#table_name::dsl;

                dsl::#table_name
                    .select(diesel::dsl::count(dsl::id))
                    .first(self.db)
                    .map_err(From::from)
            }

            #[doc=#delete_doc]
            pub fn delete_by_id(&self, id: #id_type) -> Result<usize> {
                use crate::schema::#table_name::dsl;

                assert!(id != 0, "id is zero");

                diesel::delete(dsl::#table_name.find(id))
                    .execute(self.db)
                    .map_err(From::from)
            }
      }
    })
}
