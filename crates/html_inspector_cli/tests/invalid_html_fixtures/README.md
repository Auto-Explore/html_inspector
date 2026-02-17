# Invalid HTML fixture tests

Add a new invalid HTML/XHTML file in this directory and a matching expected file:

- `case.html` or `case.xhtml`
- `case.expected.json` (a JSON array of expected **error** objects)

The test only validates `Severity::Error` (warnings/infos are ignored).

If `case.expected.json` is missing, the test will auto-generate it from the current validator output,
write it next to the fixture, and then fail so you can review/commit it.

## Expected format

`case.expected.json` is a JSON array of objects with at least:

- `code`: error code string
- `line`: 1-based line number
- `col`: 1-based column number (byte-based, like the validator output)

Optional fields:

- `at`: asserts the source starts with this string at the reported position
- `contains`: asserts the source slice for the span contains this string
