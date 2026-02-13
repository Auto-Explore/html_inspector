# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-nav-form-submit.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-nav-form-submit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Navigating iframe loading='lazy' before it is loaded: form submit</title>
<iframe name="iframe" src="support/blank.htm?src" loading="lazy" hidden></iframe>
<form action="support/blank.htm" target="iframe"></form>
<script>
const iframe = document.querySelector('iframe');
const iframeLoaded = new Promise(resolve => {
  iframe.onload = resolve;
});
const form = document.querySelector('form');
form.submit();
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
    // The "?" in the URL is there because the default method is "GET"
    // and the form data (empty here) is populated into the query.
    assert_equals(iframe.contentWindow.location.href, new URL("support/blank.htm?", location.href).href);
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-nav-form-submit.html"
}
```
