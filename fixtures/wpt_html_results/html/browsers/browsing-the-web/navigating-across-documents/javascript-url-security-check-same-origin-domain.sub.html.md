# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-security-check-same-origin-domain.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-security-check-same-origin-domain.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>javascript: URL security check for same-origin-domain but not same-origin</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="http://{{hosts[][www]}}:{{ports[http][0]}}/html/browsers/browsing-the-web/navigating-across-documents/resources/document-domain-set-to-site.sub.html"></iframe>
<script>
"use strict";
document.domain = "{{host}}";

setup({ explicit_done: true });

window.onload = () => {
  async_test(t => {
    assert_equals(frames[0].document.body.textContent, "", "before");

    window.onmessage = t.step_func_done(() => {
      assert_equals(frames[0].document.body.textContent, "new", "after");
    });

    frames[0].location.href = "javascript:parent.postMessage('done', '*'); 'new';";
  });
  done();
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “http://{{hosts[][www]}}:{{ports[http][0]}}/html/browsers/browsing-the-web/navigating-across-documents/resources/document-domain-set-to-site.sub.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 398,
        "byte_start": 235,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-security-check-same-origin-domain.sub.html"
}
```
