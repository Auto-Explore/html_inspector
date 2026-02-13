# html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/dynamic-append.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/dynamic-append.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Meta refresh applies even when dynamically appended</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#pragma-directives">

<div id="log"></div>

<script>
"use strict";
setup({ single_test: true });

const iframe = document.createElement("iframe");
let loadCount = 0;

iframe.onload = () => {
  ++loadCount;
  const iDocument = iframe.contentDocument;

  if (loadCount === 1) {
    iDocument.body.innerHTML = `<meta http-equiv="refresh" content="1; url=foo">`;
  } else if (loadCount === 2) {
    assert_equals(iDocument.body.textContent.trim(), "foo");
    done();
  }
};

iframe.src = "support/ufoo";
document.body.appendChild(iframe);
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
  "source_name": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/dynamic-append.html"
}
```
