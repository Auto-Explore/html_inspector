# html/semantics/scripting-1/the-script-element/fetch-src/empty-with-base.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/fetch-src/empty-with-base.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Script src with an empty URL</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<base href=unreachable.js>
<div id=log></div>
<script>
async_test(function(t) {
  window.unreachable = this.unreached_func("Should not load unreachable.js");
  var queued = false;
  var script = document.createElement("script");
  script.onerror = this.step_func_done(function(ev) {
    assert_equals(ev.type, "error");
    assert_false(ev.bubbles, "bubbles");
    assert_false(ev.cancelable, "cancelable");
    assert_true(ev.isTrusted, "isTrusted");
    assert_equals(ev.target, script);
    assert_true(ev instanceof Event, "instanceof Event");
    assert_class_string(ev, "Event");
    assert_true(queued, "event should not be dispatched synchronously");
  });
  script.setAttribute("src", "");
  document.body.appendChild(script);
  queued = true;
});
</script>
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
        "byte_end": 209,
        "byte_start": 183,
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
  "source_name": "html/semantics/scripting-1/the-script-element/fetch-src/empty-with-base.html"
}
```
