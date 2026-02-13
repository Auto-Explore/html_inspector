# html/semantics/embedded-content/the-embed-element/embed-document-under-content-visibility-focus.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-document-under-content-visibility-focus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTML Test: The embed element represents a document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name="assert" content="Ensure document finishes load when focus is attempted before the embed is finished loading">

<style>
.hidden { content-visibility: hidden; }
</style>
<body>
  <script>
  async_test(t => {
    addEventListener('load', t.step_func_done(() => {
      assert_true(window.childLoaded);
    }));
  }, "ensure onload happens");
  </script>
  <div class=hidden>
    <embed id=target src="embed-iframe.html">
  </div>
  <script>
    // Ensure we process style in the hidden object, which normally delays
    // load until the embed object is finished loading.
    target.focus();
  </script>
</body>
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-document-under-content-visibility-focus.html"
}
```
