## v0.4.2, 2017-05-08

* Documentation update.

* Support for Unix domain sockets on Unix-like systems.

* Support for SASL EXTERNAL binds, also limited to Unix-like systems
  for the time being, since they can only work on Unix domain socket
  connections (we can't use TLS client certs yet.)

## v0.4.1, 2017-05-06

* Fix integer parsing ([#1](https://github.com/inejge/ldap3/issues/1)).
  Active Directory length encoding triggered this bug.

* Fix the crash when parsing binary attributes ([#2](https://github.com/inejge/ldap3/issues/2)).
  The `SearchEntry`
  struct now has an additional field `bin_attrs`, containing all attributes
  which had at least one value that couldn't be converted into a `String`.
  Since it's possible that otherwise unconstrained binary attributes have
  values that _can_ be successfully converted into `String`s in a particular
  result set, the presence of such attributes should be checked for both
  in `attrs` and in `bin_attrs`.

  This is technically a breaking change, but since it isn't expected that
  any `SearchEntry` instance would've been created manually, the version
  stays at 0.4.x.

## v0.4.0, 2017-05-03

First published version.