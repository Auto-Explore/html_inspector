# html/browsers/windows/nested-browsing-contexts/frameElement-siblings.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/nested-browsing-contexts/frameElement-siblings.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>window.frameElement access to a same-origin-domain sibling</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="//{{hosts[][]}}:{{ports[http][0]}}/html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessed.html"></iframe>
<iframe src="//{{hosts[][www]}}:{{ports[http][0]}}/html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessor.html"></iframe>

<script>
"use strict";
setup({ explicit_done: true });

window.onload = () => {
  promise_test(async () => {
    frames[1].postMessage({}, "*");
    const result = await waitForMessage();

    assert_equals(result, "SecurityError");
  }, "it must give a \"SecurityError\" DOMException if the pages are different-origin domain");

  promise_test(async () => {
    document.domain = document.domain;

    frames[0].postMessage({ newDocumentDomain: document.domain }, "*");
    assert_equals(await waitForMessage(), "done");

    frames[1].postMessage({ newDocumentDomain: document.domain }, "*");
    const result = await waitForMessage();

    assert_equals(result, "HTMLIFrameElement");
  }, "it must return the iframe element if the pages are same-origin domain");

  done();
};

function waitForMessage() {
  return new Promise(resolve => {
    window.addEventListener("message", e => resolve(e.data), { once: true });
  });
}
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
  "source_name": "html/browsers/windows/nested-browsing-contexts/frameElement-siblings.sub.html"
}
```
