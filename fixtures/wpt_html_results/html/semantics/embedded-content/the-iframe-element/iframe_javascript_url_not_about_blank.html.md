# html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_not_about_blank.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_not_about_blank.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>javascript: URL in iframe src, initial src is not about:blank</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#process-the-iframe-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
setup({single_test: true});
let iframeLoaded = false;
window.javascriptUrlRan = false;
</script>
<iframe src="/resources/blank.html" onload="iframeLoaded = true; this.onload = assert_unreached;"></iframe>
<script>
onload = () => {
  const iframe = document.querySelector('iframe');
  assert_true(iframeLoaded, "iframeLoaded");
  iframe.src = "javascript:(() => { parent.javascriptUrlRan = true; })()";
  assert_false(javascriptUrlRan, "javascriptUrlRan");
  step_timeout(() => {
    assert_true(javascriptUrlRan, "javascriptUrlRan");
    done();
  }, 100); // Verify only one load event is fired on iframe
};
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_not_about_blank.html"
}
```
