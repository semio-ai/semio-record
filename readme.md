# Semio Record

This project defines the entity types used in the
[Semio Database](https://github.com/semio-ai/semio-db),
and in its clients that use the [client library](https://github.com/semio-ai/semio-client)
such as the [Semio CLI](https://github.com/semio-ai/semio-cli).

These entities have properties in common.
- They represent a versioned record in the database.
  Hence they are always referred to as **"records"**.
- They have [the same facets (see `RecordDefn`)](src/record.rs):
  - `Private`: the original description of the record,
    as defined by its author.
    It may contain restricted information,
    and it is a record that change in time (usually it is the latest),
    *i.e.* it is `Unfrozen`.
    Its dependencies are described as `UnfrozenReference`s,
    which version information can be a loose `VersionReq`.
    It can be converted into a GraphQL value for queries.
  - `Public` same as `Private` but it omits restricted information.
  - `Unfrozen`: the actual record, `Private` or `Public` are views of it.
  - `Frozen`: a record frozen in a specific version.
    Its dependencies are described as `FrozenReference`s,
    which version information is a specific `Version`.
  - `Action`: the set of operations that can be applied to a record.
    A record is the result of a sequence of actions applied to an empty shell.
- They have a type information and a type version information.
- They are meant to be serializable and deserializable using that information,
  so that their inner details do not have to be involved in the database structure.
- They can be referred to with a UUID,
  but they are not aware their UUID:
  it is the database that handles that.
- Most record types have a parent.
  When they have no parent, they are said to be "root" record types.
- They have a name. In combination with parenthood,
  names can be used to create unique paths to refer to records.
  [Semio Client does that with its notion of `Selector`](https://github.com/semio-ai/semio-client/readme.md#selector).

Each record type corresponds to a `mod` in this crate and a directory with the same name.
For each version of the record type, a sub-directory is created,
with a `mod` for each facet inside.

## Record Types

### User

[`User`](src/user.rs) represents users with access rights to the database.
This is a root record type (no parent),
and may be the parent of any other non-root record type.
It is meant to correspond to users known to the database server,
and with [controlled access rights](#access-control-lists).

### Organization

[`Organization`](src/organization.rs) represents an organization,
under which other records should be organized.
This is useful to regroup records under the same authority.

### Folder

[`Folder`](src/folder.rs) represents a folder,
that serves as an intermediary parent to other records,
in order to organize them conveniently.

### Enumeration

[`Enumeration`](src/enumeration.rs) represents an enumeration type,
with `EnumerationVariant`s capable of holding values of any other type.
Each variant has an associated UUID.

Variants are stored as an [`IndexMap<Uuid, EnumerationVariant>`][indexmap] rather
than a plain `HashMap`. This preserves the insertion order of variants across
serialization/deserialization, which keeps generated YAML record files stable —
a `HashMap` produces non-deterministic key ordering and causes spurious diffs every
time a module is rebuilt.

### Structure

[`Structure`](src/structure.rs) represents a structure type,
with `StructureField`s capable of associating values of any other type
to named and UUID-identified attributes.

Fields are stored as an [`IndexMap<Uuid, StructureField>`][indexmap] for the same
reason as `Enumeration.variants` above: to guarantee a stable serialization order.

[indexmap]: https://docs.rs/indexmap

## Common Components

### Primitive

[`Primitive`](src/ty.rs) represents a primitive type,
*but it is not a record type*.
It is used by record types representing types,
such as [`Enumeration`](#enumeration) or [`Structure`](#structure).
All records representing types can be referred to using
[`FrozenTy` or `UnfrozenTy`](src/ty.rs).

### Access Control Lists

`Private` and `Unfrozen` records hold an access control list,
defining which permissions are required to access them.
These are meant to be handled by the database,
so that to reject requests from users who were not granted the permissions.

### Freezing

Implement the [`Freezer`](src/record.rs) trait to interface with the
`freeze` methods available for `Unfrozen` records.
A freezing algorithm is already integrated in the record types,
but it needs a database (or any sort of repository of records)
to look up for matching actual tagged (and therefore `Frozen`) versions.
