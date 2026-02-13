# html/editing/dnd/datastore/datatransfer-constructor-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/datastore/datatransfer-constructor-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>DataTransfer constructor test</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
  var dt = new DataTransfer();
  assert_equals(dt.dropEffect, "none");
  assert_equals(dt.effectAllowed, "none");
  assert_equals(dt.items.length, 0);
  assert_equals(dt.types.length, 0);
}, "Verify DataTransfer constructor")
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/datastore/datatransfer-constructor-001.html"
}
```
