# html/semantics/scripting-1/the-script-element/json-module/module.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/module.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>JSON modules</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
const t = async_test();
</script>
<script type="module" onerror="t.step(() => assert_unreached(event))">
import v from "./module.json" with { type: "json" };
t.step(() => {
  assert_equals(typeof v, "object");
  assert_array_equals(Object.keys(v), ["test"]);
  assert_equals(v.test, true);
  t.done();
});
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/module.html"
}
```
