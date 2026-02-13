# html/webappapis/scripting/reporterror-cross-realm-method.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/reporterror-cross-realm-method.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>self.reportError() dispatches an "error" event for this's relevant global object</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#dom-reporterror">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
setup({ allow_uncaught_exception: true });

async_test(t => {
  window.addEventListener("error", t.unreached_func("'error' event should not be dispatched for top window!"));

  const iframe = document.createElement("iframe");
  iframe.onload = t.step_func_done(() => {
    let eventFired = false;
    const error = new TypeError("foo");
    const otherWindow = iframe.contentWindow;
    otherWindow.addEventListener("error", t.step_func(event => {
      assert_equals(event.error, error);
      eventFired = true;
    }));

    window.reportError.call(otherWindow, error);
    assert_true(eventFired);
  });
  document.body.append(iframe);
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
  "source_name": "html/webappapis/scripting/reporterror-cross-realm-method.html"
}
```
