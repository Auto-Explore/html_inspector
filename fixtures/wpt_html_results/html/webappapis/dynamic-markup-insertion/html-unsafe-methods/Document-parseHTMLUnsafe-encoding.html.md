# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-encoding.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-encoding.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="windows-1252"> <!-- intentional to make sure the results are UTF-8 anyway -->
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/9538">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- This was adapted from DOMParser-parseFromString-encoding.html -->

<script>
function assertEncoding(doc) {
  assert_equals(doc.charset, "UTF-8", "document.charset");
  assert_equals(doc.characterSet, "UTF-8", "document.characterSet");
  assert_equals(doc.inputEncoding, "UTF-8", "document.characterSet");
}

setup(() => {
  assert_equals(document.characterSet, "windows-1252", "the meta charset must be in effect, making the main document windows-1252");
});

test(() => {
  const doc = Document.parseHTMLUnsafe('');
  assertEncoding(doc);
}, 'Parse empty string');

test(() => {
  const doc = Document.parseHTMLUnsafe(`<meta charset="latin2">`);
  assertEncoding(doc);
}, "meta charset latin2");

test(() => {
  const doc = Document.parseHTMLUnsafe(`<?xml version="1.0" encoding="latin2"?><x/>`);
  assertEncoding(doc);
}, "XML declaration");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1252” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 45,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-encoding.html"
}
```
