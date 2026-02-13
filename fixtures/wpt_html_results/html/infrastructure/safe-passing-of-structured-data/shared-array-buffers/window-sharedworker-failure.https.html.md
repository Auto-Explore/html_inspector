# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-sharedworker-failure.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-sharedworker-failure.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>SharedArrayBuffer cannot cross agent clusters, shared worker edition</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
async_test(t => {
  const sw = new SharedWorker("resources/sharedworker-failure.js")
  let state = ""
  sw.port.onmessage = t.step_func(e => {
    if(e.data === "send-sw-failure") {
      sw.port.postMessage(new SharedArrayBuffer())
    } else if(e.data === "send-sw-failure-success") {
      state = "send-window-failure"
      sw.port.postMessage(state)
    } else {
      assert_unreached()
    }
  })
  sw.port.onmessageerror = t.step_func(e => {
    if(state === "send-window-failure") {
      assert_equals(e.data, null, "data")
      assert_equals(e.origin, "", "origin")
      assert_equals(e.source, null, "source")
      assert_equals(e.ports.length, 0, "ports length")
      t.done()
    } else {
      assert_unreached()
    }
  })
})
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-sharedworker-failure.https.html"
}
```
