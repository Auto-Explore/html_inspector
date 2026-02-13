# html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_loading_lazy.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_loading_lazy.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>javascript: URL in iframe src and loading="lazy"</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#process-the-iframe-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
async_test(t => {
  const iframe = document.createElement('iframe');
  iframe.src = 'javascript:"test"';
  iframe.hidden = true;
  iframe.loading = 'lazy';
  iframe.onload = t.step_func_done(() => {
    assert_false(iframe.hidden, 'Got unexpected load event on iframe while still hidden');
    assert_equals(iframe.contentDocument.body.textContent, 'test', 'Expected the document created from the javascript: URL');
  });
  document.body.append(iframe);
  window.onload = t.step_func(() => {
    assert_equals(iframe.contentDocument.body.textContent, '', 'Expected initial about:blank');
    iframe.hidden = false; // run the lazy load resumption steps, which navigates
  });
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_loading_lazy.htm"
}
```
