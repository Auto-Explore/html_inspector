# html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-initial-navigation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-initial-navigation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>Tests pageswap dispatch on initial doc navigation</title>
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
  frameWindow.onpageswap = (e) => {
      pageswapfired = true;
  }

  frameWindow.onpagehide = (e) => {
      assert_true(pageswapfired, 'pageswap fired');
      done();
  }

  frame.srcdoc = '<html></html>';
}

promise_test(async t => {
  onload = () => {
    let frame = document.createElement('iframe');
    document.body.appendChild(frame);
    runTest(frame);
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-initial-navigation.html"
}
```
