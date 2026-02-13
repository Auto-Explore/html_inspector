# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-history-replaceState.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-history-replaceState.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>History state change for iframe loading='lazy' before it is loaded: history.replaceState</title>
<iframe data-src="about:blank#replace" src="support/blank.htm?src" loading="lazy" hidden></iframe>
<script>
const iframe = document.querySelector('iframe');
const iframeLoaded = new Promise(resolve => {
  iframe.onload = resolve;
});
let replaceStateSuccess = true;
try {
  // Should have no effect on lazy-loading.
  // Per https://html.spec.whatwg.org/C#can-have-its-url-rewritten
  // only the fragment can be changed for about: URLs.
  iframe.contentWindow.history.replaceState(null, "", iframe.dataset.src);
} catch(ex) {
  replaceStateSuccess = false;
}
const locationAfterReplaceState = iframe.contentWindow.location.href;
iframe.hidden = false;
</script>
<!-- Loading testharness.js here is intentional to reproduce a bug in WebKit. -->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
setup({single_test: true});
assert_true(replaceStateSuccess);
assert_equals(locationAfterReplaceState, new URL("about:blank#replace", location.href).href);
iframeLoaded.then(() => {
  // No timeout needed in this test because history.replaceState() doesn't navigate.
  assert_equals(iframe.contentWindow.location.href, new URL("support/blank.htm?src", location.href).href);
  done();
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-history-replaceState.html"
}
```
