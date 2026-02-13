# html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/allow-scripts-flag-changing-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/allow-scripts-flag-changing-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Meta refresh of the original iframe is not blocked if moved into a sandboxed iframe</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#attr-meta-http-equiv-refresh">

<div id="log"></div>

<script>
"use strict";
setup({ single_test: true });

const sourceIFrame = document.createElement("iframe");

const destIFrame = document.createElement("iframe");
destIFrame.setAttribute("sandbox", "allow-same-origin");

let sourceLoadCount = 0;
let destLoadCount = 0;

sourceIFrame.onload = () => {
  ++sourceLoadCount;

  if (sourceLoadCount === 2) {
    assert_equals(sourceIFrame.contentDocument.body.textContent.trim(), "foo");
    done();
  }

  maybeStartTest();
};

destIFrame.onload = () => {
  ++destLoadCount;

  if (destLoadCount === 2) {
    assert_unreached("The iframe into which the meta was moved must not refresh");
  }

  maybeStartTest();
};

function maybeStartTest() {
  if (sourceLoadCount === 1 && destLoadCount === 1) {
    const meta = sourceIFrame.contentDocument.querySelector("meta");
    destIFrame.contentDocument.body.appendChild(meta);
  }
}

sourceIFrame.src = "support/refresh.sub.html?input=" + encodeURIComponent("1; url=foo");
destIFrame.src = "support/ufoo";

document.body.appendChild(sourceIFrame);
document.body.appendChild(destIFrame);
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
  "source_name": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/allow-scripts-flag-changing-2.html"
}
```
