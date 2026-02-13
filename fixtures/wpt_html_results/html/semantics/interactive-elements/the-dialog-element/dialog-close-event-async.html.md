# html/semantics/interactive-elements/the-dialog-element/dialog-close-event-async.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-close-event-async.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML//EN">
<html>
<head>
<title>dialog element: close()</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#the-dialog-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<dialog id="d1" open>
  <p>foobar</p>
  <button>OK</button>
</dialog>
<script>
  var d1 = document.getElementById('d1'),
      t = async_test("close() fires a close event"),
      was_queued = false;

  d1.onclose = t.step_func_done(function(e) {
    assert_true(was_queued, "close event should be queued");
    assert_true(e.isTrusted, "close event is trusted");
    assert_false(e.bubbles, "close event doesn't bubble");
    assert_false(e.cancelable, "close event is not cancelable");
  });

  t.step(function() {
    d1.close();
    was_queued = true;
  })
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.doctype.not_html5",
      "message": "Obsolete doctype. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-close-event-async.html"
}
```
