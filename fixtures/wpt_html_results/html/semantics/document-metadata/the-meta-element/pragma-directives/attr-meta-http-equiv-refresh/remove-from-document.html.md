# html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/remove-from-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/remove-from-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>A meta must refresh the original document even if it was removed.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#attr-meta-http-equiv-refresh">

<div id="log"></div>

<script>
"use strict";
setup({ single_test: true });

const sourceIFrame = document.createElement("iframe");
let sourceLoadCount = 0;

sourceIFrame.onload = () => {
  ++sourceLoadCount;

  if (sourceLoadCount === 2) {
    assert_equals(sourceIFrame.contentDocument.body.textContent.trim(), "foo");
    done();
  }

  maybeStartTest();
};

function maybeStartTest() {
  if (sourceLoadCount === 1) {
    sourceIFrame.contentDocument.querySelector("meta").remove();
  }
}

sourceIFrame.src = "support/refresh.sub.html?input=" + encodeURIComponent("1; url=foo");

document.body.appendChild(sourceIFrame);
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
  "source_name": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/remove-from-document.html"
}
```
