# html/browsers/sandboxing/sandbox-navigation-timing.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-navigation-timing.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Sandbox Navigation Timing</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<html></html>
<script>
  const sandboxUrl = location.pathname.substring(0, location.pathname.lastIndexOf('/') + 1) + 'sandbox-navigation-timing-iframe.tentative.html';
  async_test(t => {
    const iframe = document.createElement('iframe');
    iframe.src = sandboxUrl;
    document.body.appendChild(iframe); // Navigation starts; value of sandbox flags locked on.
    // This should not affect the sandbox value used for both about:blank document
    // and the final document in iframe.
    iframe.sandbox = 'allow-scripts';
    const iframeAboutBlankDocument = iframe.contentDocument;

    iframe.onload = t.step_func(() => {
      const iframeAboutBlankContents = iframeAboutBlankDocument.querySelectorAll('body');
      assert_equals(iframeAboutBlankContents[0].tagName, "BODY",
        "about:blank document's contents should still be accessible");

      iframe.contentWindow.postMessage("is iframe sandboxed?", "*");
    });
    window.onmessage = t.step_func_done(e => {
      assert_equals(e.data.result, 'iframe not sandboxed');
    });
  }, 'setting sandbox attribute should not affect current document in iframe');
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
  "source_name": "html/browsers/sandboxing/sandbox-navigation-timing.tentative.html"
}
```
