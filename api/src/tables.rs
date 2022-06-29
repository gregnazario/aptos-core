// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    context::Context,
    failpoint::fail_point,
    metrics::metrics,
    param::{LedgerVersionParam, TableHandleParam},
    state::State,
    version::Version,
};
use aptos_api_types::TableItemRequest;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

// GET /tables/<table_handle>/item
pub fn get_table_item(context: Context) -> BoxedFilter<(impl Reply,)> {
    warp::path!("tables" / TableHandleParam / "item")
        .and(warp::post())
        .and(warp::body::content_length_limit(
            context.content_length_limit(),
        ))
        .and(warp::body::json::<TableItemRequest>())
        .and(context.filter())
        .and(warp::query::<Version>())
        .map(|handle, body, ctx, version: Version| (version.version, handle, body, ctx))
        .untuple_one()
        .and_then(handle_get_table_item)
        .with(metrics("get_table_item"))
        .boxed()
}

async fn handle_get_table_item(
    ledger_version: Option<LedgerVersionParam>,
    handle: TableHandleParam,
    body: TableItemRequest,
    context: Context,
) -> Result<impl Reply, Rejection> {
    fail_point("endpoint_get_table_item")?;
    Ok(State::new(ledger_version, context)?.table_item(handle.parse("table handle")?, body)?)
}
