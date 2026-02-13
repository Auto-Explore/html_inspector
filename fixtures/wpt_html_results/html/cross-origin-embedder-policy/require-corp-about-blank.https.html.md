# html/cross-origin-embedder-policy/require-corp-about-blank.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/require-corp-about-blank.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>

promise_test(t => {
  return new Promise(resolve => {
    window.addEventListener("DOMContentLoaded", resolve);
  });
}, "Wait for the DOM to be built.");

promise_test(async t => {
  let iframe = document.createElement("iframe");
  let iframe_loaded =  new Promise(resolve => iframe.onload = resolve);
  iframe.src = "about:blank";
  document.body.appendChild(iframe);

  // The about:blank document can load.
  await iframe_loaded;
  assert_not_equals(iframe.contentDocument, null);
}, "about:blank can always be embedded by a 'require-corp' document");

promise_test(async t => {
  let iframe_C = document.createElement("iframe");
  let iframe_B = document.createElement("iframe");
  iframe_B.src = "about:blank";
  iframe_C.src = "/common/blank.html";
  let iframe_B_loaded = new Promise(resolve => iframe_B.onload = resolve);
  document.body.appendChild(iframe_B);

  // The about:blank frame must be able to load.
  await iframe_B_loaded;
  assert_not_equals(iframe_B.contentDocument, null);
  iframe_B.contentDocument.body.appendChild(iframe_C);


  // The document nested under about:blank must not load because it does not
  // specify the Cross-Origin-Embedder-Policy: require-corp header.
  // An error page must be displayed instead.
  // See https://github.com/whatwg/html/issues/125 for why a timeout is used
  // here. Long term all network error handling should be similar and have a
  // reliable event.
  assert_equals(iframe_C.contentWindow.location.href, "about:blank");
  assert_not_equals(iframe_C.contentDocument, null);
  await t.step_wait(() => iframe_C.contentDocument === null);
}, "A(B(C)) A=require-corp, B=about:blank, C=no-require-corp => C can't load");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/cross-origin-embedder-policy/require-corp-about-blank.https.html"
}
```
