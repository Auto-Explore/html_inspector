# html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_initial_insertion.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_initial_insertion.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>javascript: URL in iframe src, initial insertion check</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#process-the-iframe-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
setup({single_test: true});
let iframeLoaded = false;
window.javascriptUrlRan = 0;
</script>
<iframe src="javascript:(() => { parent.javascriptUrlRan++; })()" onload="iframeLoaded = true; this.onload = assert_unreached;"></iframe>
<script>
onload = () => {
  const iframe = document.querySelector('iframe');
  assert_true(iframeLoaded, "iframeLoaded");
  iframe.src = iframe.src + ";";
  assert_equals(javascriptUrlRan, 1, "javascriptUrlRan");
  step_timeout(() => {
    assert_equals(javascriptUrlRan, 2, "javascriptUrlRan");
    done();
  }, 100); // Verify only one load event is fired on iframe
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “javascript:(() => { parent.javascriptUrlRan++; })()” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 524,
        "byte_start": 396,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_initial_insertion.html"
}
```
