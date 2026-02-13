# html/semantics/embedded-content/the-iframe-element/iframe_navigate_ancestor-1.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_navigate_ancestor-1.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  async_test(t => {
    window.addEventListener('message', t.step_func_done(e => {
      assert_equals(e.data, "can navigate");
    }));
  }, "A => B => B: B should be able to navigate B.");
</script>
<iframe src="https://{{hosts[][www]}}:{{ports[https][0]}}/html/semantics/embedded-content/the-iframe-element/support/iframe-tried-to-be-navigated-by-its-child.html"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “https://{{hosts[][www]}}:{{ports[https][0]}}/html/semantics/embedded-content/the-iframe-element/support/iframe-tried-to-be-navigated-by-its-child.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 520,
        "byte_start": 355,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_navigate_ancestor-1.sub.html"
}
```
