# html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/not-in-shadow-tree.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/not-in-shadow-tree.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Meta refresh only applies while in the document tree, not in a shadow tree</title>
<meta name="timeout" content="long" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#pragma-directives">

<div id="log"></div>
<script>
"use strict";
setup({ single_test: true });

const iframe = document.createElement("iframe");
iframe.src = "support/ufoo";

let loadCount = 0;

iframe.onload = () => {
  ++loadCount;
  const iDocument = iframe.contentDocument;

  if (loadCount === 1) {
    const div = iDocument.createElement("div");
    assert_true('attachShadow' in div, 'attachShadow support');
    const shadowRoot = div.attachShadow({ mode: "open" });
    shadowRoot.innerHTML = `<meta http-equiv="refresh" content="1; url=foo">`;
    iDocument.body.appendChild(div);

    // Want to make sure no refreshes happen
    step_timeout(done, 3000);
  } else {
    assert_unreached("Got more than 1 load event");
  }
};

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
  "source_name": "html/semantics/document-metadata/the-meta-element/pragma-directives/attr-meta-http-equiv-refresh/not-in-shadow-tree.html"
}
```
