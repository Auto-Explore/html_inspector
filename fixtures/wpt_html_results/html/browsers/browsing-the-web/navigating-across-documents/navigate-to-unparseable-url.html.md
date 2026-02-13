# html/browsers/browsing-the-web/navigating-across-documents/navigate-to-unparseable-url.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/navigate-to-unparseable-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>location.href unparseable URL throws a SyntaxError DOMException</title>
<link rel="help" href="https://html.spec.whatwg.org/#the-location-interface:dom-location-href-2">
<link rel="help" href="https://html.spec.whatwg.org/#following-hyperlinks-2">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
const kUnparseableURL = self.origin + ":notaport/common/blank.html";

promise_test(async t => {
  const win = window.open("/common/blank.html");
  t.add_cleanup(() => {
    win.close();
  });

  await new Promise(resolve => {
    win.onload = resolve;
  });

  assert_throws_dom("SyntaxError", win.DOMException, () => {
    win.location.href = kUnparseableURL;
  }, "location.href setter throws a SyntaxError DOMException");
}, "location.href setter throws a SyntaxError DOMException for unparseable " +
   "URLs");

promise_test(async t => {
  const win = window.open("/common/blank.html");
  t.add_cleanup(() => {
    win.close();
  });

  await new Promise(resolve => {
    win.onload = resolve;
  });

  // If the newly-opened window tries to navigate, fail the test.
  const failPromise = new Promise((resolve, reject) => {
    win.onpagehide = () =>
        reject(new Error("Navigation was attempted to unparseable URL"));
  });

  // A promise to wait on to confirm the newly-opened window did not navigate.
  const successPromise = new Promise(resolve => {
    t.step_timeout(resolve, 2000);
  });

  const a = win.document.createElement('a');
  a.href = kUnparseableURL;
  win.document.body.append(a);
  a.click();

  return Promise.race([successPromise, failPromise]);
}, "<a> tag navigate fails for unparseable URLs");
</script>
</body>
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/navigate-to-unparseable-url.html"
}
```
