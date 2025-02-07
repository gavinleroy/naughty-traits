# Naughty Traits

Please note that this repository was forked from [@weiznich](https://github.com/weiznich) and includes many great examples they collected from the community. 
Added are examples that I want to track—which may one day be upstreamed, should there be interest. I am *not* funded by a Rust Foundation project grant, and 
this information has been removed from the README and repo name.

## Call for participation

Please submit examples of bad error messages in the context of trait-heavy crates 
as issue or PR (with minimal example) to this repository.


## Test cases

| crate           | test case                     | error type                                       |
|-----------------|-------------------------------|--------------------------------------------------|
| [uom]           | [type_mismatch.rs]            | type mismatch                                    |
| [typed_builder] | [mismatch.rs]                 | type mismatch + missing free standing function   |
| [easy_ml]       | [recursion.rs]                | type recursion                                   |
| [diesel]        | [bad_insertable_field.rs]     | trait not implemented + misleading wildcard impl |
| [diesel]        | [bad_sql_query.rs]            | trait not implemented                            |
| [diesel]        | [invalid_query.rs]            | traits not implemented + "duplicated errors"     |
| [diesel]        | [queryable_order_mismatch.rs] | trait not implemented with large types           |
| [chumsky]       | [json.rs]                     | associated type mismatch                         |
| [bevy]          | [system_mismatch.rs]          | trait not implemented + HRTB error               |
| [axum]          | [argument_not_extractor.rs]   | debug_handler                                    |
| [axum]          | [extract_self_mut.rs]         | debug_handler                                    |
| [axum]          | [extract_self_ref.rs]         | debug_handler                                    |
| [axum]          | [generics.rs]                 | debug_handler                                    |
| [axum]          | [invalid_attrs.rs]            | debug_handler                                    |
| [axum]          | [missing_deserialize.rs]      | trait not implemented                            |
| [axum]          | [multiple_body_extractors.rs] | debug_handler                                    |
| [axum]          | [multiple_paths.rs]           | debug_handler                                    |
| [axum]          | [not_a_function.rs]           | debug_handler                                    |
| [axum]          | [not_async.rs]                | debug_handler                                    |
| [axum]          | [not_send.rs]                 | debug_handler                                    |
| [axum]          | [request_not_last.rs]         | debug_handler                                    |
| [axum]          | [too_many_extractors.rs]      | debug_handler                                    |
| [axum]          | [wrong_return_type.rs]        | debug_handler                                    |
| [entrait]       | [missing_impl_deep.rs]        | trait not implemented                            |


[uom]: https://crates.io/crates/uom
[typed_builder]: https://crates.io/crates/typed_builder
[easy_ml]: https://crates.io/crates/easy_ml
[diesel]: https://crates.io/crates/diesel
[chumsky]: https://crates.io/crates/chumsky
[bevy]: https://crates.io/crates/bevy
[axum]: https://crates.io/crates/axum
[entrait]: https://crates.io/crates/entrait

[type_mismatch.rs]: #uom_type_mismatch
[mismatch.rs]: #typed_builder_mismatch
[recursion.rs]: #easy_ml_recursion
[bad_insertable_field.rs]: #diesel_bad_insertable
[bad_sql_query.rs]: #diesel_bad_sql_query
[invalid_query.rs]: #diesel_invalid_query
[queryable_order_mismatch.rs]: #diesel_queryable
[json.rs]: #chumsky_json
[system_mismatch.rs]: #bevy_system_mismatch
[argument_not_extractor.rs]: #axum_argument_not_extractor
[extract_self_mut.rs]: #axum_extract_self_mut
[extract_self_ref.rs]: #axum_extract_self_ref
[generics.rs]: #axum_generics
[invalid_attrs.rs]: #axum_invalid_attrs
[missing_deserialize.rs]: #axum_missing_deserialize
[multiple_body_extractors.rs]: #axum_multiple_body_extractors
[multiple_paths.rs]: #axum_multiple_paths
[not_a_function.rs]: #axum_not_a_function
[not_async.rs]: #axum_not_async
[not_send.rs]: #axum_not_send
[request_not_last.rs]: #axum_not_last
[too_many_extractors.rs]: #axum_too_many_extractors.rs]
[wrong_return_type.rs]: #axum_wrong_return_type
[missing_impl_deep.rs]: #entrait_missing_impl_deep

### uom

#### type_mismatch.rs <a name="uom_type_mismatch></a>

Versions:

| version | link (code)                            | link (error message)                           | change since last version |
|---------|----------------------------------------|------------------------------------------------|---------------------------|
| 1       | [type_mismatch.rs][type_mismatch.rs-1] | [type_mismatch.stderr][type_mismatch.stderr-1] |                           |

[type_mismatch.rs-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/uom/type_mismatch.rs
[type_mismatch.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/uom/type_mismatch.stderr


### typed_builder

Typed builder generates hidden types/deprecating messages to improve its error messages

#### mismatch.rs <a name="typed_builder_mismatch"></a>
 
 Versions: 
 
 | version | link (code)                  | link (error message)                 | change since last version |
 |---------|------------------------------|--------------------------------------|---------------------------|
 | 1       | [mismatch.rs][mismatch.rs-1] | [mismatch.stderr][mismatch.stderr-1] |                           |

[mismatch.rs-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/typed_builder/mismatch.rs
[mismatch.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/typed_builder/mismatch.stderr

### entrait

#### missing_impl_deep.rs <a name="entrait_missing_impl_deep"></a>

Versions:

 | version | link (code)                    | link (error message)                   | change since last version |
 |---------|--------------------------------|----------------------------------------|---------------------------|
 | 1       | [missing_impl_deep.rs][missing_impl_deep.rs-1] | [missing_impl_deep.stderr][missing_impl_deep.stderr-1] |                           |

[missing_impl_deep.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/b90e741dc3a4d1615f087b6011ceb3eb65c64ee7/test_cases/tests/entrait/missing_impl_deep.rs
[missing_impl_deep.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/b90e741dc3a4d1615f087b6011ceb3eb65c64ee7/test_cases/tests/entrait/missing_impl_deep.stderr


### easy_ml

#### recursion.rs <a name="easy_ml_recursion"></a>

Versions:

 | version | link (code)                    | link (error message)                   | change since last version |
 |---------|--------------------------------|----------------------------------------|---------------------------|
 | 1       | [recursion.rs][recursion.rs-1] | [recursion.stderr][recursion.stderr-1] |                           |

[recursion.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/easy_ml/recursion.rs
[recursion.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/easy_ml/recursion.stderr


### diesel

#### bad_insertable_field.rs <a name = "diesel_bad_insertable"></a>

 | version | link (code)                                    | link (error message)                                  | change since last version |
 |---------|------------------------------------------------|-------------------------------------------------------|---------------------------|
 | 1       | [bad_insertable_field.rs][bad_insertable.rs-1] | [bad_insertable_field.stderr][bad_insertable.stderr-1] |                           |
 | 2       | [bad_insertable_field.rs][bad_insertable.rs-2] | [bad_insertable_field.stderr][bad_insertable.stderr-2] | https://github.com/diesel-rs/diesel/pull/3228 improves the spans for certain trait bounds so that compiler errors point to the corresponding struct fields instead  of the derive                          |


[bad_insertable.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/diesel/bad_insertable_field.rs
[bad_insertable.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/diesel/bad_insertable_field.stderr
[bad_insertable.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/7322e4ba58f7a47b27bb7e88e9ded056fdb79e99/test_cases/tests/diesel/bad_insertable_field.rs
[bad_insertable.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/7322e4ba58f7a47b27bb7e88e9ded056fdb79e99/test_cases/tests/diesel/bad_insertable_field.stderr


#### bad_sql_query.rs <a name = "diesel_bad_sql_query"></a>

 | version | link (code)                            | link (error message)                           | change since last version                                                                                                                         |
 |---------|----------------------------------------|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
 | 1       | [bad_sql_query.rs][bad_sql_query.rs-1] | [bad_sql_query.stderr][bad_sql_query.stderr-1] |                                                                                                                                                   |
 | 2       | [bad_sql_query.rs][bad_sql_query.rs-2] | [bad_sql_query.stderr][bad_sql_query.stderr-2] | Add a `#[rustc_on_unimplemented]` on the corresponding trait. https://github.com/diesel-rs/diesel/commit/958391a3e793e409d0a925e0cc2317726c2d84b2 |



[bad_sql_query.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/bad_sql_query.rs
[bad_sql_query.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/diesel/bad_sql_query.stderr
[bad_sql_query.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/83fd28b310b7f33a630cb851f7fd8a5cc610d3fc/test_cases/tests/bad_sql_query.rs
[bad_sql_query.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/83fd28b310b7f33a630cb851f7fd8a5cc610d3fc/test_cases/tests/diesel/bad_sql_query.stderr


#### invalid_query.rs <a name="diesel_invalid_query"></a>

 | version | link (code)                            | link (error message)                           | change since last version                                                                                                                         |
 |---------|----------------------------------------|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
 | 1       | [invalid_query.rs][invalid_query.rs-1] | [invalid_query.stderr][invalid_query.stderr-1] |                                                                                                                                                   |
 | 2       | [invalid_query.rs][invalid_query.rs-2] | [invalid_query.stderr][invalid_query.stderr-2] | Add a `#[rustc_on_unimplemented]` on the corresponding trait. https://github.com/diesel-rs/diesel/commit/958391a3e793e409d0a925e0cc2317726c2d84b2 |

[invalid_query.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/diesel/invalid_query.rs
[invalid_query.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/diesel/invalid_query.stderr
[invalid_query.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/83fd28b310b7f33a630cb851f7fd8a5cc610d3fc/test_cases/tests/diesel/invalid_query.rs
[invalid_query.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/83fd28b310b7f33a630cb851f7fd8a5cc610d3fc/test_cases/tests/diesel/invalid_query.stderr


#### queryable_order_mismatch.rs <a name = "diesel_queryable"></a>

 | version | link (code)                    | link (error message)                   | change since last version |
 |---------|--------------------------------|----------------------------------------|---------------------------|
 | 1       | [queryable_order_mismatch.rs][queryable_order_mismatch.rs-1] | [queryable_order_mismatch.stderr][queryable_order_mismatch.stderr-1] 

[queryable_order_mismatch.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/diesel/queryable_order_mismatch.rs
[queryable_order_mismatch.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/diesel/queryable_order_mismatch.stderr

### chumsky

#### json.rs <a name="chumsky_json"></a>


 | version | link (code)          | link (error message)          | change since last version |
 |---------|----------------------|-------------------------------|---------------------------|
 | 1       | [json.rs][json.rs-1] | [json.stderr][json.stderr-1]  |                           |

[json.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/chumsky/json.rs
[json.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/chumsky/json.stderr

### bevy

#### system_mismatch.rs <a name="bevy_system_mismatch"></a>


 | version | link (code)                                | link (error message)                               | change since last version                                                                                                   |
 |---------|--------------------------------------------|----------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
 | 1       | [system_mismatch.rs][system_mismatch.rs-1] | [system_mismatch.stderr][system_mismatch.stderr-1] |                                                                                                                             |
 | 2       | [system_mismatch.rs][system_mismatch.rs-2] | [system_mismatch.stderr][system_mismatch.stderr-2] | https://github.com/bevyengine/bevy/pull/5786, which introduces `#[rustc_on_unimplemented]` attributes in multiple locations |
|         |                                            |                                                    |                                                                                                                             |

[system_mismatch.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/bevy/system_mismatch.rs
[system_mismatch.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/bevy/system_mismatch.stderr
[system_mismatch.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/e69f4f7e5bfa8faeeeb43369b206c32c45c3b5d0/test_cases/tests/bevy/system_mismatch.rs
[system_mismatch.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/e69f4f7e5bfa8faeeeb43369b206c32c45c3b5d0/test_cases/tests/bevy/system_mismatch.stderr

### axum

axum provides a `#[debug_handler]` attribute which emits better error messages is some cases

#### argument_not_extractor.rs <a name="axum_argument_not_extractor"></a>

 | version | link (code)                                              | link (error message)                                             | change since last version                                           |
 |---------|----------------------------------------------------------|------------------------------------------------------------------|---------------------------------------------------------------------|
 | 1       | [argument_not_extractor.rs][argument_not_extractor.rs-1] | [argument_not_extractor.stderr][argument_not_extractor.stderr-1] |                                                                     |
 | 2       | [argument_not_extractor.rs][argument_not_extractor.rs-2] | [argument_not_extractor.stderr][argument_not_extractor.stderr-2] | [Fixes to the error spans of `#[debug_handler]`][debug_handler_fix] |
 | 3       | [argument_not_extractor.rs][argument_not_extractor.rs-3] | [argument_not_extractor.stderr][argument_not_extractor.stderr-3] | [Point to `#[debug_handler]`][point_to_debug_handler]               |

[argument_not_extractor.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/argument_not_extractor.rs
[argument_not_extractor.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/argument_not_extractor.stderr
[argument_not_extractor.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/3d98d3813285b30260ecc65b40ae8b340ac35cae/test_cases/tests/axum/argument_not_extractor.rs
[argument_not_extractor.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/3d98d3813285b30260ecc65b40ae8b340ac35cae/test_cases/tests/axum/argument_not_extractor.stderr
[debug_handler_fix]:https://github.com/weiznich/axum/commit/d5d076dae495d5f76c364182e070d26ebe4972b8
[argument_not_extractor.rs-3]:https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/argument_not_extractor.rs
[argument_not_extractor.stderr-3]: https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/argument_not_extractor.stderr
[point_to_debug_handler]: https://github.com/weiznich/axum/commit/a151aac96a8569c59df2de2c0ae3d645ab1c6430

#### extract_self_ref <a name="axum_extract_self_ref"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [extract_self_ref.rs][extract_self_ref.rs-1] | [extract_self_ref.stderr][extract_self_ref.stderr-1] |                           |

[extract_self_ref.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/extract_self_ref.rs
[extract_self_ref.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/extract_self_ref.stderr

#### extract_self_mut <a name="axum_extract_self_mut"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [extract_self_mut.rs][extract_self_mut.rs-1] | [extract_self_mut.stderr][extract_self_mut.stderr-1] |                           |

[extract_self_mut.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/extract_self_mut.rs
[extract_self_mut.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/extract_self_mut.stderr

#### generics <a name="axum_generics"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [generics.rs][generics.rs-1] | [generics.stderr][generics.stderr-1] |                           |

[generics.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/generics.rs
[generics.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/generics.stderr

#### invalid_attrs <a name="axum_invalid_attrs"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [invalid_attrs.rs][invalid_attrs.rs-1] | [invalid_attrs.stderr][invalid_attrs.stderr-1] |                           |

[invalid_attrs.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/invalid_attrs.rs
[invalid_attrs.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/invalid_attrs.stderr

#### missing_deserialize.rs <a name="axum_missing_deserialize"></a>

 | version | link (code)                                        | link (error message)                                       | change since last version                             |
 |---------|----------------------------------------------------|------------------------------------------------------------|-------------------------------------------------------|
 | 1       | [missing_deserialize.rs][missing_deserialize.rs-1] | [missing_deserialize.stderr][missing_deserialize.stderr-1] |                                                       |
 | 2       | [missing_deserialize.rs][missing_deserialize.rs-2] | [missing_deserialize.stderr][missing_deserialize.stderr-2] | [Point to `#[debug_handler]`][point_to_debug_handler] |


[missing_deserialize.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/missing_deserialize.rs
[missing_deserialize.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/missing_deserialize.stderr
[point_to_debug_handler]: https://github.com/weiznich/axum/commit/a151aac96a8569c59df2de2c0ae3d645ab1c6430
[missing_deserialize.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/missing_deserialize.rs
[missing_deserialize.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/missing_deserialize.stderr

#### multiple_body_extractors <a name="axum_multiple_body_extractors"></a>

 | version | link (code)                                                  | link (error message)                                                 | change since last version                                           |
 |---------|--------------------------------------------------------------|----------------------------------------------------------------------|---------------------------------------------------------------------|
 | 1       | [multiple_body_extractors.rs][multiple_body_extractors.rs-1] | [multiple_body_extractors.stderr][multiple_body_extractors.stderr-1] |                                                                     |
 | 2       | [multiple_body_extractors.rs][multiple_body_extractors.rs-2] | [multiple_body_extractors.stderr][multiple_body_extractors.stderr-2] | [Fixes to the error spans of `#[debug_handler]`][debug_handler_fix] |
 | 3       | [multiple_body_extractors.rs][multiple_body_extractors.rs-3] | [multiple_body_extractors.stderr][multiple_body_extractors.stderr-3] | [Point to `#[debug_handler]`][point_to_debug_handler]               |


[multiple_body_extractors.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/multiple_body_extractors.rs
[multiple_body_extractors.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/multiple_body_extractors.stderr
[multiple_body_extractors.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/3d98d3813285b30260ecc65b40ae8b340ac35cae/test_cases/tests/axum/multiple_body_extractors.rs
[multiple_body_extractors.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/3d98d3813285b30260ecc65b40ae8b340ac35cae/test_cases/tests/axum/multiple_body_extractors.stderr
[multiple_body_extractors.rs-3]:https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/multiple_body_extractors.rs
[multiple_body_extractors.stderr-3]: https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/multiple_body_extractors.stderr
[point_to_docs]: https://github.com/weiznich/axum/commit/a151aac96a8569c59df2de2c0ae3d645ab1c6430
[debug_handler_fix]:https://github.com/weiznich/axum/commit/d5d076dae495d5f76c364182e070d26ebe4972b8

#### multiple_paths <a name="axum_multiple_paths"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [multiple_paths.rs][multiple_paths.rs-1] | [multiple_paths.stderr][multiple_paths.stderr-1] |                           |

[multiple_paths.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/multiple_paths.rs
[multiple_paths.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/multiple_paths.stderr

#### not_a_function <a name="axum_not_a_function"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [not_a_function.rs][not_a_function.rs-1] | [not_a_function.stderr][not_a_function.stderr-1] |                           |

[not_a_function.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/not_a_function.rs
[not_a_function.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/not_a_function.stderr

#### not_async <a name="axum_not_async"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [not_async.rs][not_async.rs-1] | [not_async.stderr][not_async.stderr-1] |                           |

[not_async.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/not_async.rs
[not_async.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/not_async.stderr

#### not_send <a name="axum_not_send"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [not_send.rs][not_send.rs-1] | [not_send.stderr][not_send.stderr-1] |                           |

[not_send.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/not_send.rs
[not_send.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/not_send.stderr


#### request_not_last <a name="axum_request_not_last"></a>

 | version | link (code)                                  | link (error message)                                 | change since last version                                           |
 |---------|----------------------------------------------|------------------------------------------------------|---------------------------------------------------------------------|
 | 1       | [request_not_last.rs][request_not_last.rs-1] | [request_not_last.stderr][request_not_last.stderr-1] |                                                                     |
 | 2       | [request_not_last.rs][request_not_last.rs-2] | [request_not_last.stderr][request_not_last.stderr-2] | [Fixes to the error spans of `#[debug_handler]`][debug_handler_fix] |
 | 3       | [request_not_last.rs][request_not_last.rs-3] | [request_not_last.stderr][request_not_last.stderr-3] | [Point out as part of the error message, that `Request` always needs to be the last handler argument][improve_error_message_for_request_not_last]                                                                    |

[request_not_last.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/request_not_last.rs
[request_not_last.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/request_not_last.stderr
[improve_error_message_for_request_not_last]: https://github.com/weiznich/axum/commit/a151aac96a8569c59df2de2c0ae3d645ab1c6430
[debug_handler_fix]:https://github.com/weiznich/axum/commit/d5d076dae495d5f76c364182e070d26ebe4972b8
[request_not_last.rs-2]:https://github.com/weiznich/rust-foundation-community-grant/blob/3d98d3813285b30260ecc65b40ae8b340ac35cae/test_cases/tests/axum/request_not_last.rs
[request_not_last.stderr-2]: https://github.com/weiznich/rust-foundation-community-grant/blob/3d98d3813285b30260ecc65b40ae8b340ac35cae/test_cases/tests/axum/request_not_last.stderr
[request_not_last.rs-3]:https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/request_not_last.rs
[request_not_last.stderr-3]: https://github.com/weiznich/rust-foundation-community-grant/blob/a7d2bf8a408580d6a3b047fe194096bf39479719/test_cases/tests/axum/request_not_last.stderr


#### too_many_extractors <a name="axum_too_many_extractors"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [too_many_extractors.rs][too_many_extractors.rs-1] | [too_many_extractors.stderr][too_many_extractors.stderr-1] |                           |

[too_many_extractors.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/too_many_extractors.rs
[too_many_extractors.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/too_many_extractors.stderr

#### wrong_return_type.rs <a name="axum_wrong_return_type"></a>

 | version | link (code)                                    | link (error message)                                   | change since last version |
 |---------|------------------------------------------------|--------------------------------------------------------|---------------------------|
 | 1       | [wrong_return_type.rs][wrong_return_type.rs-1] | [wrong_return_type.stderr][wrong_return_type.stderr-1] |                           |

[wrong_return_type.rs-1]:https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/wrong_return_type.rs
[wrong_return_type.stderr-1]: https://github.com/weiznich/rust-foundation-community-grant/blob/883de46cbea5873bcc4af60e47f872efaa77a2b7/test_cases/tests/axum/wrong_return_type.stderr
