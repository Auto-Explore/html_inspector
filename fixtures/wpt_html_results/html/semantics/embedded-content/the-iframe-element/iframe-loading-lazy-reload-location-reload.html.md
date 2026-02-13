# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-reload-location-reload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-reload-location-reload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reloading iframe loading='lazy' before it is loaded: location.reload</title>
<iframe src="support/blank.htm?src" loading="lazy" hidden></iframe>
<script>
const iframe = document.querySelector('iframe');
const iframeLoaded = new Promise(resolve => {
  iframe.onload = resolve;
});
// Should have no effect on lazy-loading
iframe.contentWindow.location.reload();
iframe.hidden = false;
</script>
<!-- Loading testharness.js here is intentional to reproduce a bug in WebKit. -->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
setup({single_test: true});
iframeLoaded.then(() => {
  // Need a timeout to detect failure when there are two navigations.
  step_timeout(() => {
    assert_equals(iframe.contentWindow.location.href, new URL("support/blank.htm?src", location.href).href);
    done();
  }, 1000);
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-reload-location-reload.html"
}
```
