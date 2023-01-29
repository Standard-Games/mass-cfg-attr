mass-cfg-attr-tests-tests
=========================

This passing tests in mass-cfg-attr-tests should only pass because mass-cfg-attr did its job

To test this we copy each of the passing tests into this project and remove any line with mass-cfg-attr. We then test
that the build fails. Because Rust error messages vary by version, the compiler output should not be tested, therefore
you should only run tests in this director with:

```shell
$ TRYBUILD=overwrite cargo test
```

Tests in this directory are always overwritten by the ones in passing, so it's safe to rerun these after changes.
