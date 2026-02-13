# html/browsers/browsing-the-web/history-traversal/same-url.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/same-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>Test same-URL navigation and its effects on history</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<iframe src=resources/a.html></iframe>
<script>
async_test((t) => {
  let state = "begin"
  let initialLength = history.length

  self[0].frameElement.onload = t.step_func(() => {
    if(state === "b first") {
      assert_equals(history.length, initialLength + 1, state)

      state = "c first"
      navigateFrameAfterDelay(t, "resources/c.html")
    } else if (state === "c first") {
      assert_equals(history.length, initialLength + 2, state)

      state = "a second"
      history.back(2)
    } else if (state === "a second") {
      assert_equals(history.length, initialLength + 2, state)

      state = "a third"
      navigateFrameAfterDelay(t, "resources/a.html")
    } else if (state === "a third") {
      assert_equals(history.length, initialLength + 2, state)
      t.done()
    }
  })
  onload = t.step_func(() => {
    assert_equals(state, "begin")

    state = "b first"

    navigateFrameAfterDelay(t, "resources/b.html")
  })
})

function navigateFrameAfterDelay(t, url) {
  // Delay to avoid triggering the "replace" behavior which occurs if
  // the page isn't yet completely loaded, which only occurs after the
  // load event handlers have finished:
  // https://html.spec.whatwg.org/#location-object-navigate
  // https://html.spec.whatwg.org/#the-end:completely-finish-loading
  t.step_timeout(() => {
    self[0].location = url
  }, 0)
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 7,
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/same-url.html"
}
```
