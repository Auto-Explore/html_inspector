# html/cross-origin-embedder-policy/sandbox.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/sandbox.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<div id=log></div>
<script>
async_test(t => {
  window.addEventListener("message", t.step_func_done(({ data }) => {
    assert_equals(data.origin, "null");
    assert_true(data.sameOriginWithoutCORP, "Request to same-origin resource without CORP did not fail");
    assert_true(data.sameOriginWithSameOriginCORP, "Request to same-origin resource with same-origin CORP did not fail");
    assert_true(data.sameOriginWithCrossOriginCORP, "Request to same-origin resource with cross-origin CORP did not succeed");
    assert_true(data.crossOriginWithCrossOriginCORP, "Request to cross-origin resource with cross-origin CORP did not succeed");
  }));

  const origins = get_host_info();
  const frame = document.createElement("iframe");
  const nothingCrossOriginCORP = new URL("resources/nothing-cross-origin-corp.js", window.location).pathname;
  const nothingSameOriginCORP = new URL("resources/nothing-same-origin-corp.txt", window.location).pathname;
  frame.sandbox = "allow-scripts";
  frame.srcdoc = `<script>
const data = { sameOriginWithoutCORP: false,
               sameOriginWithSameOriginCORP: false,
               sameOriginWithCrossOriginCORP: false,
               crossOriginWithCrossOriginCORP: false,
               origin: self.origin };
function record(promise, token, expectation) {
  return promise.then(() => data[token] = expectation, () => data[token] = !expectation);
}
Promise.all([
  record(fetch("/common/blank.html", { mode: "no-cors" }), "sameOriginWithoutCORP", false),
  record(fetch("${nothingSameOriginCORP}", { mode: "no-cors" }), "sameOriginWithSameOriginCORP", false),
  record(fetch("${nothingCrossOriginCORP}", { mode: "no-cors" }), "sameOriginWithCrossOriginCORP", true),
  record(fetch("${origins.HTTPS_NOTSAMESITE_ORIGIN}${nothingCrossOriginCORP}", { mode: "no-cors" }), "crossOriginWithCrossOriginCORP", true)
]).then(() => parent.postMessage(data, "*"));
<\/script>`;
  document.body.append(frame);
}, "Cross-Origin-Embedder-Policy and sandbox");
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
  "source_name": "html/cross-origin-embedder-policy/sandbox.https.html"
}
```
