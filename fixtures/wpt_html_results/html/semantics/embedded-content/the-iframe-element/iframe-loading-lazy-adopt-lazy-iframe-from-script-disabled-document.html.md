# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-adopt-lazy-iframe-from-script-disabled-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-adopt-lazy-iframe-from-script-disabled-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Test lazy loading iframe remains lazy loading originally created
         in a script-disabled document, then got adopted to a regular document</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<script>
  const t = async_test("Test lazy loading iframe remains lazy loading originally created" +
                       "in a script-disabled document, then got adopted to a regular document");

  const lazyIFrameInScriptDisabledDocumentOnLoad =
    t.unreached_func("lazy loading iframe shoudn't be loaded because it's not visible");

  window.addEventListener("load", t.step_func(() => {
    const parser = new DOMParser();
    const remoteDoc = `
        <!DOCTYPE html>
        <html>
          <iframe
            id="lazyIFrameInScriptDisabledDocument"
            src="resources/subframe.html"
            loading="lazy"
            width="600"
            height="400"
            style="display: none"
          >
          Your browser does not support iframes.
          </iframe>
        </html>`;

    const htmlDoc = parser.parseFromString(remoteDoc, "text/html");
    var newIframe = htmlDoc.getElementById("lazyIFrameInScriptDisabledDocument");
    newIframe.onload = lazyIFrameInScriptDisabledDocumentOnLoad;

    document.body.appendChild(newIframe);

    // Give some time to the "load" event to fire.
    t.step_timeout(t.step_func_done(), 2000);
  }));
</script>
</html>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-adopt-lazy-iframe-from-script-disabled-document.html"
}
```
