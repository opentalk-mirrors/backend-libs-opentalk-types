// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Pagination types
//!
//! Great blogposts are:
//! - <https://phauer.com/2015/restful-api-design-best-practices/>
//! - <https://phauer.com/2018/web-api-pagination-timestamp-id-continuation-token/>

mod cursor;
mod page_pagination_query;

pub use cursor::Cursor;
pub use page_pagination_query::PagePaginationQuery;
