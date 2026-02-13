# html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>Tests pageswap dispatch on iframe Documents</title>
<link rel="author" title="Khushal Sagar"  href="mailto:khushalsagar@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
setup({explicit_done: true});

function runTest(frame) {
  let frameWindow = frame.contentWindow;

  let pageswapfired = false;
  let expectedUrl = frameWindow.location.href + '?new';
  frameWindow.onpageswap = (e) => {
      assert_equals(e.activation.entry.url, expectedUrl, 'activation url incorrect in pageswap');
      assert_equals(e.activation.navigationType, "push", 'navigation type incorrect in pageswap');
      assert_equals(e.activation.from, frameWindow.navigation.currentEntry, 'from entry incorrect in pageswap');
      assert_false(e.activation.entry.sameDocument, 'new entry must be cross-document');
      pageswapfired = true;
  }

  frameWindow.onpagehide = (e) => {
      assert_true(pageswapfired, 'pageswap not fired');
      done();
  }

  frame.src = expectedUrl;
}

promise_test(async t => {
  onload = () => {
    let frame = document.createElement('iframe');
    frame.src = "/resources/blank.html";
    frame.onload = () => {
      frame.contentWindow.requestAnimationFrame(() => {
        runTest(frame);
      });
    }
    document.body.appendChild(frame);
  };
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-iframe.html"
}
```
