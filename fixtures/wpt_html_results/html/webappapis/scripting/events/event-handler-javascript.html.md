# html/webappapis/scripting/events/event-handler-javascript.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-javascript.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Event handler with labels</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body onload="javascript:
  for (var i = 0; i < 2; ++i) {
    for (var j = 0; j < 2; ++j) {
      t.step(function() {
        assert_equals(i, 0);
        assert_equals(j, 0);
      });
      break javascript;
    }
  }
  t.done();
">
<div id="log"></div>
<script>
var t = async_test("Event handlers starting with 'javascript:' should treat that as a label.");
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
  "source_name": "html/webappapis/scripting/events/event-handler-javascript.html"
}
```
