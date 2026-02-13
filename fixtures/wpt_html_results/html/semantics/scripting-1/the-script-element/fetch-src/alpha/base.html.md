# html/semantics/scripting-1/the-script-element/fetch-src/alpha/base.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/fetch-src/alpha/base.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Script src with a base URL</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<base href=../beta/>
<div id=log></div>
<script>
function do_test(path) {
  test(function() {
    assert_equals(path, "beta");
  });
}
</script>
<script src=test.js></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 201,
        "byte_start": 181,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/fetch-src/alpha/base.html"
}
```
