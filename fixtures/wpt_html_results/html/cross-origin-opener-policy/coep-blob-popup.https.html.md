# html/cross-origin-opener-policy/coep-blob-popup.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coep-blob-popup.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Cross-Origin-Opener-Policy and Cross-Origin-Embedder-Policy: blob URL popup</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/get-host-info.sub.js></script>
<script src=../cross-origin-embedder-policy/resources/script-factory.js></script>
<script>
["window.open()", "<a>", "<a rel=noopener>"].forEach(type => {
  promise_test(t => {
    const origins = get_host_info();
    const id = `tut mir leid ${type}`;
    const blob = new Blob([`<script>${createScript(origins.ORIGIN, origins.HTTPS_REMOTE_ORIGIN, "channel", id)}<\/script>`], {type: "text/html"});
    const blobURL = URL.createObjectURL(blob);
    const bc = new BroadcastChannel(id);

    if (type === "window.open()") {
      const popup = window.open(blobURL);
      t.add_cleanup(() => popup.close());
      popup.onload = t.step_func(() => {
        assert_equals(popup.opener, window);
        assert_equals(popup.location.href, blobURL);
        assert_equals(popup.document.URL, blobURL);
        assert_equals(popup.origin, window.origin);
      });
    } else {
      const a = document.createElement("a");
      a.target = type;
      if (type === "<a rel=noopener>") {
        a.rel = "noopener";
      }
      a.href = blobURL;
      a.click();
    }

    return new Promise(resolve => {
      bc.onmessage = t.step_func(({ data }) => {
        assert_equals(data.id, id);
        assert_equals(data.origin, window.origin);
        assert_true(data.sameOriginNoCORPSuccess, "Same-origin without CORP did not succeed");
        assert_true(data.crossOriginNoCORPFailure, "Cross-origin without CORP did not fail");
        if (type === "<a rel=noopener>") {
          assert_false(data.opener, 'opener');
        } else {
          assert_true(data.opener, 'opener');
        }
        resolve();
      });
    });
  }, `COOP+COEP blob URL popup: ${type}`);
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
  "source_name": "html/cross-origin-opener-policy/coep-blob-popup.https.html"
}
```
