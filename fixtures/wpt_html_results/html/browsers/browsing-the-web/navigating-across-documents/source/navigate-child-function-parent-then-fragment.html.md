# html/browsers/browsing-the-web/navigating-across-documents/source/navigate-child-function-parent-then-fragment.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/source/navigate-child-function-parent-then-fragment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>
  Set location from a parent, then do a fragment navigation from within the
  frame.
</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe></iframe>
<script>
  promise_test(async test => {
    // Wait for the DOM to be ready before inserting an <iframe> into it.
    await new Promise(resolve => { onload = resolve });
    // Insert an <iframe> and wait for a dummy document to be loaded into it.
    let iframe = document.createElement("iframe");
    iframe.src = "support/dummy.html";
    let iframe_loaded = new Promise(resolve => { iframe.onload = resolve });
    document.body.appendChild(iframe);
    await iframe_loaded;
    // The referrer is the main frame's URL since it initiated the iframe
    // creation.
    assert_equals(iframe.contentDocument.referrer, document.URL);
    // Do a fragment navigation from the frame, which will fire the
    // 'hashchange' function.
    let hash_changed = new Promise(resolve =>  {
      iframe.contentWindow.onhashchange = resolve
    });
    let navigateScript = iframe.contentDocument.createElement("script");
    navigateScript.innerHTML = "location.href = '#foo'";
    iframe.contentDocument.body.appendChild(navigateScript);
    await hash_changed;
    // The referrer stays the same, even when the last navigation was
    // initiated by the iframe (instead of the main frame document).
    assert_equals(iframe.contentDocument.referrer, document.URL);
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/source/navigate-child-function-parent-then-fragment.html"
}
```
